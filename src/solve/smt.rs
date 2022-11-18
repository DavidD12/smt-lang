use crate::combine::*;
use crate::problem::*;
use std::collections::HashMap;
use z3::ast::Ast;

pub enum Z3Solver<'a> {
    Solve(z3::Solver<'a>),
    Optimize(z3::Optimize<'a>),
}

impl<'a> Z3Solver<'a> {
    pub fn assert(&self, ast: &z3::ast::Bool<'a>) {
        match self {
            Z3Solver::Solve(solver) => solver.assert(ast),
            Z3Solver::Optimize(solver) => solver.assert(ast),
        }
    }

    pub fn maximize(&self, ast: &z3::ast::Int<'a>) {
        match self {
            Z3Solver::Solve(_) => {}
            Z3Solver::Optimize(solver) => solver.maximize(ast),
        }
    }

    pub fn minimize(&self, ast: &z3::ast::Int<'a>) {
        match self {
            Z3Solver::Solve(_) => {}
            Z3Solver::Optimize(solver) => solver.minimize(ast),
        }
    }

    pub fn check(&self) -> z3::SatResult {
        match self {
            Z3Solver::Solve(solver) => solver.check(),
            Z3Solver::Optimize(solver) => solver.check(&[]),
        }
    }

    pub fn get_model(&self) -> Option<z3::Model> {
        match self {
            Z3Solver::Solve(solver) => solver.get_model(),
            Z3Solver::Optimize(solver) => solver.get_model(),
        }
    }
}

impl<'a> std::fmt::Display for Z3Solver<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Z3Solver::Solve(solver) => write!(f, "{}", solver),
            Z3Solver::Optimize(solver) => write!(f, "{}", solver),
        }
    }
}

pub struct Smt<'a> {
    problem: &'a Problem,
    //
    _cfg: &'a z3::Config,
    ctx: &'a z3::Context,
    solver: Z3Solver<'a>,
    // Structure
    struc_sort: HashMap<StructureId, z3::Sort<'a>>,
    struc_instances: HashMap<InstanceId, z3::ast::Datatype<'a>>,
    struc_attributes: HashMap<AttributeId<StructureId>, z3::FuncDecl<'a>>,
    struc_methods: HashMap<MethodId<StructureId>, z3::FuncDecl<'a>>,
    // Class
    class_instances_sort: HashMap<ClassId, z3::Sort<'a>>,
    class_instances: HashMap<InstanceId, z3::ast::Datatype<'a>>,
    class_datatype_sort: HashMap<ClassId, z3::DatatypeSort<'a>>,
    class_sort_variants: HashMap<ClassId, HashMap<ClassId, usize>>,
    class_attributes: HashMap<AttributeId<ClassId>, z3::FuncDecl<'a>>,
    class_methods: HashMap<MethodId<ClassId>, z3::FuncDecl<'a>>,
    // Variable
    bool_variables: HashMap<VariableId, z3::ast::Bool<'a>>,
    int_variables: HashMap<VariableId, z3::ast::Int<'a>>,
    real_variables: HashMap<VariableId, z3::ast::Real<'a>>,
    datatype_variables: HashMap<VariableId, z3::ast::Datatype<'a>>,
    // Function
    functions: HashMap<FunctionId, z3::FuncDecl<'a>>,
    // Constraint
    constraints: HashMap<ConstraintId, z3::ast::Bool<'a>>,
}

impl<'a> Smt<'a> {
    pub fn new(problem: &'a Problem, cfg: &'a z3::Config, ctx: &'a z3::Context) -> Self {
        let solver = match problem.search() {
            Search::Solve => Z3Solver::Solve(z3::Solver::new(&ctx)),
            Search::Optimize(_, _) => Z3Solver::Optimize(z3::Optimize::new(&ctx)),
        };
        Self {
            problem,
            _cfg: cfg,
            ctx,
            solver,
            //
            struc_sort: HashMap::new(),
            struc_instances: HashMap::new(),
            struc_attributes: HashMap::new(),
            struc_methods: HashMap::new(),
            //
            class_instances_sort: HashMap::new(),
            class_instances: HashMap::new(),
            class_datatype_sort: HashMap::new(),
            class_sort_variants: HashMap::new(),
            class_attributes: HashMap::new(),
            class_methods: HashMap::new(),
            //
            bool_variables: HashMap::new(),
            int_variables: HashMap::new(),
            real_variables: HashMap::new(),
            datatype_variables: HashMap::new(),
            functions: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    //------------------------- -------------------------

    pub fn problem(&self) -> &Problem {
        self.problem
    }

    pub fn solver(&self) -> &Z3Solver {
        &self.solver
    }

    //------------------------- Sort -------------------------

    fn to_sort(&self, typ: &Type) -> z3::Sort<'a> {
        match typ {
            Type::Bool => z3::Sort::bool(self.ctx),
            Type::Int => z3::Sort::int(self.ctx),
            Type::Real => z3::Sort::real(self.ctx),
            Type::Interval(_, _) => z3::Sort::int(self.ctx),
            Type::Structure(id) => self.struc_sort.get(id).unwrap().clone(),
            Type::Class(id) => self.class_datatype_sort.get(id).unwrap().sort.clone(),
            Type::Unresolved(_, _) => panic!(),
            Type::Undefined => panic!(),
        }
    }

    //------------------------- Structure -------------------------

    fn declare_structure(&mut self, structure: &Structure) {
        let instances = structure.instances(self.problem);
        let names = instances
            .iter()
            .map(|i| self.problem.get(*i).unwrap().name().into())
            .collect::<Vec<_>>();
        let (sort, consts, _testers) =
            z3::Sort::enumeration(self.ctx, structure.name().into(), &names);
        // Sort
        self.struc_sort.insert(structure.id(), sort);
        // Instance
        for (id, f) in instances.iter().zip(consts.into_iter()) {
            self.struc_instances
                .insert(*id, f.apply(&[]).as_datatype().unwrap());
        }
    }

    fn declare_structure_elements(&mut self, structure: &Structure) {
        // Attribute
        for attribute in structure.attributes().iter() {
            let sort = self.structure_sort(structure.id());
            let name = format!("_{}__{}", structure.name(), attribute.name());
            let fun = z3::FuncDecl::new(self.ctx, name, &[sort], &self.to_sort(attribute.typ()));
            self.struc_attributes.insert(attribute.id(), fun);
        }
        // Method
        for method in structure.methods().iter() {
            let sort = self.structure_sort(structure.id());
            let name = format!("_{}__{}", structure.name(), method.name());
            let params = method
                .parameters()
                .iter()
                .map(|p| self.to_sort(p.typ()))
                .collect::<Vec<_>>();
            let mut v = Vec::new();
            v.push(sort);
            v.extend(params.iter());
            let met = z3::FuncDecl::new(self.ctx, name, &v, &self.to_sort(method.typ()));
            self.struc_methods.insert(method.id(), met);
        }
    }

    fn define_structure(&mut self, structure: &Structure) {
        // Attribute
        for attribute in structure.attributes().iter() {
            if let Some(expr) = attribute.expr() {
                for instance in structure.instances(self.problem).iter() {
                    // Self
                    let self_expr = Expr::StrucSelf(structure.id(), None);
                    let inst_expr = Expr::Instance(*instance, None);
                    let expr = expr.substitute(&self_expr, &inst_expr);
                    //
                    let att_expr = Expr::StrucAttribute(Box::new(inst_expr), attribute.id(), None);
                    let ass = Expr::Binary(Box::new(att_expr), BinOp::Eq, Box::new(expr), None);
                    let ass = ass.type_inference(self.problem);
                    self.solver.assert(&self.to_bool(&ass));
                }
            }
        }
        // Method
        for method in structure.methods().iter() {
            if let Some(expr) = method.expr() {
                for instance in structure.instances(self.problem).iter() {
                    // Self
                    let self_expr = Expr::StrucSelf(structure.id(), None);
                    let inst_expr = Expr::Instance(*instance, None);
                    let expr = expr.substitute(&self_expr, &inst_expr.clone());
                    //
                    let l = Box::new(Expr::StrucMetCall(
                        Box::new(inst_expr),
                        method.id(),
                        method
                            .parameters()
                            .iter()
                            .map(|p| Expr::Parameter(p.clone()))
                            .collect(),
                        None,
                    ));
                    let r = Box::new(expr.clone());
                    let e = Expr::Binary(l, BinOp::Eq, r, None);
                    let exprs = combine_all(self.problem, method.parameters(), &e);
                    for e in exprs.iter() {
                        let e = e.type_inference(self.problem);
                        self.solver.assert(&self.to_bool(&e));
                    }
                }
            }
        }
    }

    fn declare_structures(&mut self) {
        for x in self.problem.structures().iter() {
            self.declare_structure(x);
        }
        for x in self.problem.structures().iter() {
            self.declare_structure_elements(x);
        }
    }

    fn define_structures(&mut self) {
        for x in self.problem.structures().iter() {
            self.define_structure(x);
        }
    }

    fn structure_sort(&self, id: StructureId) -> &z3::Sort<'a> {
        &self.struc_sort[&id]
    }

    pub fn struc_instance(&self, id: InstanceId) -> &z3::ast::Datatype<'a> {
        &self.struc_instances[&id]
    }

    //------------------------- Class -------------------------

    fn declare_class_instances(&mut self, class: &Class) {
        let instances = class.instances(self.problem);
        let names = instances
            .iter()
            .map(|i| self.problem.get(*i).unwrap().name().into())
            .collect::<Vec<_>>();
        let name = format!("_{}_", class.name());
        let (sort, consts, _testers) = z3::Sort::enumeration(self.ctx, name.into(), &names);
        // Sort
        self.class_instances_sort.insert(class.id(), sort);
        // Instance
        for (id, f) in instances.iter().zip(consts.into_iter()) {
            self.class_instances
                .insert(*id, f.apply(&[]).as_datatype().unwrap());
        }
    }

    fn declare_class(&mut self, class: &Class) {
        let mut builder = z3::DatatypeBuilder::new(self.ctx, class.name());
        let mut map = HashMap::new();
        // Current Class
        let name = format!("_{}", class.name());
        let field_name = "objects";
        let field_sort = self.class_instances_sort.get(&class.id()).unwrap();
        builder = builder.variant(
            &name,
            vec![(field_name, z3::DatatypeAccessor::Sort(field_sort.clone()))],
        );
        map.insert(class.id(), map.len());
        // Sub Classes
        for id in class.direct_sub_classes(self.problem).into_iter() {
            let c = self.problem.get(id).unwrap();
            let name = format!("_{}_{}_", class.name(), c.name());
            let field_name = "objects";
            let field_sort = &self.class_datatype_sort.get(&id).unwrap().sort;
            builder = builder.variant(
                &name,
                vec![(field_name, z3::DatatypeAccessor::Sort(field_sort.clone()))],
            );
            map.insert(id, map.len());
        }
        let datatype_sort = builder.finish();
        self.class_sort_variants.insert(class.id(), map);
        self.class_datatype_sort.insert(class.id(), datatype_sort);
    }

    fn declare_class_elements(&mut self, class: &Class) {
        // Attribute
        for attribute in class.attributes().iter() {
            let sort = self.class_sort(class.id());
            let name = format!("_{}__{}", class.name(), attribute.name());
            let fun = z3::FuncDecl::new(self.ctx, name, &[sort], &self.to_sort(attribute.typ()));
            self.class_attributes.insert(attribute.id(), fun);
        }
        // Method
        for method in class.methods().iter() {
            let sort = self.class_sort(class.id());
            let name = format!("_{}__{}", class.name(), method.name());
            let params = method
                .parameters()
                .iter()
                .map(|p| self.to_sort(p.typ()))
                .collect::<Vec<_>>();
            let mut v = Vec::new();
            v.push(sort);
            v.extend(params.iter());
            let met = z3::FuncDecl::new(self.ctx, name, &v, &self.to_sort(method.typ()));
            self.class_methods.insert(method.id(), met);
        }
    }

    fn define_class(&mut self, class: &Class) {
        // Attribute
        for attribute in class.attributes().iter() {
            if let Some(expr) = attribute.expr() {
                for instance in class.instances(self.problem).iter() {
                    // Self
                    let self_expr = Expr::ClassSelf(class.id(), None);
                    let inst_expr = Expr::Instance(*instance, None);
                    let expr = expr.substitute(&self_expr, &inst_expr);
                    //
                    let att_expr = Expr::ClassAttribute(Box::new(inst_expr), attribute.id(), None);
                    let ass = Expr::Binary(Box::new(att_expr), BinOp::Eq, Box::new(expr), None);
                    let ass = ass.type_inference(self.problem);
                    self.solver.assert(&self.to_bool(&ass));
                }
            }
        }
        // Method
        for method in class.methods().iter() {
            if let Some(expr) = method.expr() {
                for instance in class.instances(self.problem).iter() {
                    // Self
                    let self_expr = Expr::ClassSelf(class.id(), None);
                    let inst_expr = Expr::Instance(*instance, None);
                    let expr = expr.substitute(&self_expr, &inst_expr.clone());
                    //
                    let l = Box::new(Expr::ClassMetCall(
                        Box::new(inst_expr),
                        method.id(),
                        method
                            .parameters()
                            .iter()
                            .map(|p| Expr::Parameter(p.clone()))
                            .collect(),
                        None,
                    ));
                    let r = Box::new(expr.clone());
                    let e = Expr::Binary(l, BinOp::Eq, r, None);
                    let exprs = combine_all(self.problem, method.parameters(), &e);
                    for e in exprs.iter() {
                        let e = e.type_inference(self.problem);
                        self.solver.assert(&self.to_bool(&e));
                    }
                }
            }
        }
    }

    fn declare_classes(&mut self) {
        let classes = self
            .problem
            .classes_ordered()
            .iter()
            .rev()
            .map(|c| self.problem.get(*c).unwrap())
            .collect::<Vec<_>>();
        for x in classes.iter() {
            self.declare_class_instances(x);
        }
        for x in classes.iter() {
            self.declare_class(x);
        }
        for x in classes.iter() {
            self.declare_class_elements(x);
        }
    }

    fn define_classes(&mut self) {
        for x in self.problem.classes().iter() {
            self.define_class(x);
        }
    }

    pub fn class_sort(&self, id: ClassId) -> &z3::Sort<'a> {
        &self.class_datatype_sort.get(&id).unwrap().sort
    }

    pub fn class_object(&self, instance: &Instance) -> z3::ast::Datatype<'a> {
        let inst = self.class_instances.get(&instance.id()).unwrap();
        let class_id = instance.typ().class().unwrap();
        let variant_id = self
            .class_sort_variants
            .get(&class_id)
            .unwrap()
            .get(&class_id)
            .unwrap();
        let value = self.class_datatype_sort.get(&class_id).unwrap().variants[*variant_id]
            .constructor
            .apply(&[inst]);
        value.as_datatype().unwrap()
    }

    //------------------------- Variable -------------------------

    fn declare_variable(&mut self, variable: &Variable) {
        match variable.typ() {
            Type::Bool => {
                let v = z3::ast::Bool::new_const(self.ctx, variable.name());
                self.bool_variables.insert(variable.id(), v);
            }
            Type::Int => {
                let v = z3::ast::Int::new_const(self.ctx, variable.name());
                self.int_variables.insert(variable.id(), v);
            }
            Type::Real => {
                let v = z3::ast::Real::new_const(self.ctx, variable.name());
                self.real_variables.insert(variable.id(), v);
            }
            Type::Interval(min, max) => {
                let v = z3::ast::Int::new_const(self.ctx, variable.name());
                self.int_variables.insert(variable.id(), v);
                let v = self.int_variable(variable.id());
                self.solver
                    .assert(&v.ge(&z3::ast::Int::from_i64(self.ctx, *min as i64)));
                self.solver
                    .assert(&v.le(&z3::ast::Int::from_i64(self.ctx, *max as i64)));
            }
            Type::Structure(id) => {
                let sort = self.struc_sort.get(&id).unwrap();
                let v = z3::ast::Datatype::new_const(self.ctx, variable.name(), sort);
                self.datatype_variables.insert(variable.id(), v);
            }
            Type::Class(id) => {
                let sort = &self.class_datatype_sort.get(&id).unwrap().sort;
                let v = z3::ast::Datatype::new_const(self.ctx, variable.name(), sort);
                self.datatype_variables.insert(variable.id(), v);
            }
            Type::Unresolved(_, _) => panic!(),
            Type::Undefined => panic!(),
        }
    }

    fn define_variable(&mut self, variable: &Variable) {
        if let Some(expr) = variable.expr() {
            let v = Expr::Variable(variable.id(), None);
            let e = &Expr::Binary(Box::new(v), BinOp::Eq, Box::new(expr.clone()), None);
            let e = e.type_inference(self.problem);
            self.solver.assert(&self.to_bool(&e));
        }
    }

    fn declare_variables(&mut self) {
        for v in self.problem.variables().iter() {
            self.declare_variable(v);
        }
    }

    fn define_variables(&mut self) {
        for v in self.problem.variables().iter() {
            self.define_variable(v);
        }
    }

    pub fn bool_variable(&self, id: VariableId) -> &z3::ast::Bool<'a> {
        self.bool_variables.get(&id).unwrap()
    }

    pub fn int_variable(&self, id: VariableId) -> &z3::ast::Int<'a> {
        self.int_variables.get(&id).unwrap()
    }

    pub fn real_variable(&self, id: VariableId) -> &z3::ast::Real<'a> {
        self.real_variables.get(&id).unwrap()
    }

    pub fn datatype_variable(&self, id: VariableId) -> &z3::ast::Datatype<'a> {
        self.datatype_variables.get(&id).unwrap()
    }

    //------------------------- Function -------------------------

    fn declare_function(&mut self, function: &Function) {
        let params = function
            .parameters()
            .iter()
            .map(|p| self.to_sort(p.typ()))
            .collect::<Vec<_>>();
        let params = params.iter().collect::<Vec<_>>();
        let fun = z3::FuncDecl::new(
            self.ctx,
            function.name(),
            &params,
            &self.to_sort(function.typ()),
        );
        self.functions.insert(function.id(), fun);
    }

    fn define_function(&mut self, function: &Function) {
        if let Some(expr) = function.expr() {
            let l = Box::new(Expr::FunctionCall(
                function.id(),
                function
                    .parameters()
                    .iter()
                    .map(|p| Expr::Parameter(p.clone()))
                    .collect(),
                None,
            ));
            let r = Box::new(expr.clone());
            let e = Expr::Binary(l, BinOp::Eq, r, None);
            let exprs = combine_all(self.problem, function.parameters(), &e);
            for e in exprs.iter() {
                let e = e.type_inference(self.problem);
                self.solver.assert(&self.to_bool(&e));
            }
        }
    }

    fn declare_functions(&mut self) {
        for f in self.problem.functions().iter() {
            self.declare_function(f);
        }
    }

    fn define_functions(&mut self) {
        for f in self.problem.functions().iter() {
            self.define_function(f);
        }
    }

    pub fn function(&self, id: FunctionId) -> &z3::FuncDecl<'a> {
        self.functions.get(&id).unwrap()
    }

    //------------------------- Constraint -------------------------

    fn declare_constraint(&mut self, constraint: &Constraint) {
        let c = z3::ast::Bool::new_const(self.ctx, constraint.name());
        self.constraints.insert(constraint.id(), c);
    }

    fn define_constraint(&mut self, constraint: &Constraint) {
        let c = self.constraint(constraint.id());
        let e = constraint.expr().type_inference(self.problem);
        let e = self.to_bool(&e);
        self.solver.assert(&c._eq(&e));
        self.solver.assert(&c);
    }

    fn add_constraints(&mut self) {
        for c in self.problem.constraints().iter() {
            self.declare_constraint(c);
        }
        for c in self.problem.constraints().iter() {
            self.define_constraint(c);
        }
    }

    pub fn constraint(&self, id: ConstraintId) -> &z3::ast::Bool<'a> {
        self.constraints.get(&id).unwrap()
    }

    //------------------------- Search -------------------------

    pub fn add_search(&mut self) {
        match self.problem.search() {
            Search::Solve => {}
            Search::Optimize(e, minimize) => {
                let e = e.type_inference(self.problem);
                let e = self.to_int(&e);
                if *minimize {
                    self.solver.minimize(&e);
                } else {
                    self.solver.maximize(&e);
                }
            }
        }
    }

    //------------------------- Expr -------------------------

    fn to_dynamic(&self, expr: &Expr) -> z3::ast::Dynamic<'a> {
        let t = &expr.typ(self.problem);
        if t.is_bool() {
            let x = self.to_bool(expr);
            z3::ast::Dynamic::new(self.ctx, x.get_z3_ast())
        } else if t.is_integer() {
            let x = self.to_int(expr);
            z3::ast::Dynamic::new(self.ctx, x.get_z3_ast())
        } else if t.is_real() {
            let x = self.to_real(expr);
            z3::ast::Dynamic::new(self.ctx, x.get_z3_ast())
        } else if t.is_structure() {
            let x = self.to_datatype(expr);
            z3::ast::Dynamic::new(self.ctx, x.get_z3_ast())
        } else if t.is_class() {
            let x = self.to_datatype(expr);
            z3::ast::Dynamic::new(self.ctx, x.get_z3_ast())
        } else {
            panic!()
        }
    }

    fn fun_call(&self, id: FunctionId, parameters: &Vec<Expr>) -> z3::ast::Dynamic<'a> {
        let fun = self.functions.get(&id).unwrap();
        let params = parameters
            .iter()
            .map(|p| self.to_dynamic(p))
            .collect::<Vec<_>>();
        let params = params
            .iter()
            .map(|p| p as &dyn z3::ast::Ast)
            .collect::<Vec<_>>();
        fun.apply(&params)
    }

    fn struc_met_call(
        &self,
        expr: &Expr,
        id: MethodId<StructureId>,
        parameters: &Vec<Expr>,
    ) -> z3::ast::Dynamic<'a> {
        let meth = self.struc_methods.get(&id).unwrap();
        let mut params = Vec::new();
        params.push(self.to_dynamic(expr));
        params.extend(parameters.iter().map(|p| self.to_dynamic(p)));
        let params = params
            .iter()
            .map(|p| p as &dyn z3::ast::Ast)
            .collect::<Vec<_>>();
        meth.apply(&params)
    }

    fn class_met_call(
        &self,
        expr: &Expr,
        id: MethodId<ClassId>,
        parameters: &Vec<Expr>,
    ) -> z3::ast::Dynamic<'a> {
        let meth = self.class_methods.get(&id).unwrap();
        let mut params = Vec::new();
        params.push(self.to_dynamic(expr));
        params.extend(parameters.iter().map(|p| self.to_dynamic(p)));
        let params = params
            .iter()
            .map(|p| p as &dyn z3::ast::Ast)
            .collect::<Vec<_>>();
        meth.apply(&params)
    }

    pub fn to_bool(&self, expr: &Expr) -> z3::ast::Bool<'a> {
        match expr {
            Expr::BoolValue(value, _) => z3::ast::Bool::from_bool(self.ctx, *value),
            Expr::Prefix(op, e, _) => match op {
                PreOp::Not => {
                    let e = self.to_bool(e);
                    z3::ast::Bool::not(&e)
                }
                _ => panic!(),
            },
            Expr::Binary(l, op, r, _) => {
                let t = &l.typ(self.problem);
                if t.is_bool() {
                    let l = self.to_bool(l);
                    let r = self.to_bool(r);
                    match op {
                        BinOp::Eq => l._eq(&r),
                        BinOp::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        BinOp::And => z3::ast::Bool::and(self.ctx, &[&l, &r]),
                        BinOp::Or => z3::ast::Bool::or(self.ctx, &[&l, &r]),
                        BinOp::Implies => z3::ast::Bool::implies(&l, &r),
                        _ => panic!(),
                    }
                } else if t.is_integer() {
                    let l = self.to_int(l);
                    let r = self.to_int(r);
                    match op {
                        BinOp::Eq => l._eq(&r),
                        BinOp::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        BinOp::Lt => l.lt(&r),
                        BinOp::Le => l.le(&r),
                        BinOp::Ge => l.ge(&r),
                        BinOp::Gt => l.gt(&r),
                        _ => panic!(),
                    }
                } else if t.is_real() {
                    let l = self.to_real(l);
                    let r = self.to_real(r);
                    match op {
                        BinOp::Eq => l._eq(&r),
                        BinOp::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        BinOp::Lt => l.lt(&r),
                        BinOp::Le => l.le(&r),
                        BinOp::Ge => l.ge(&r),
                        BinOp::Gt => l.gt(&r),
                        _ => panic!(),
                    }
                } else if t.is_structure() {
                    let l = self.to_datatype(l);
                    let r = self.to_datatype(r);
                    match op {
                        BinOp::Eq => l._eq(&r),
                        BinOp::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        _ => panic!(),
                    }
                } else if t.is_class() {
                    let l = self.to_datatype(l);
                    let r = self.to_datatype(r);
                    match op {
                        BinOp::Eq => l._eq(&r),
                        BinOp::Ne => z3::ast::Bool::not(&l._eq(&r)),
                        _ => panic!(),
                    }
                } else {
                    panic!("{:?}", t)
                }
            }
            Expr::Variable(id, _) => self.bool_variable(*id).clone(),
            Expr::FunctionCall(id, parameters, _) => {
                self.fun_call(*id, parameters).as_bool().unwrap()
            }
            Expr::StrucAttribute(e, id, _) => {
                let fun = self.struc_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_bool().unwrap()
            }
            Expr::StrucMetCall(e, id, parameters, _) => {
                self.struc_met_call(e, *id, parameters).as_bool().unwrap()
            }
            Expr::ClassAttribute(e, id, _) => {
                let fun = self.class_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_bool().unwrap()
            }
            Expr::ClassMetCall(e, id, parameters, _) => {
                self.class_met_call(e, *id, parameters).as_bool().unwrap()
            }
            Expr::IfThenElse(c, t, l, e, _) => {
                let c = self.to_bool(c);
                let t = self.to_bool(t);
                let l = l
                    .iter()
                    .map(|(x, y)| (self.to_bool(x), self.to_bool(y)))
                    .collect::<Vec<_>>();
                let e = self.to_bool(e);
                let mut res = e;
                for (x, y) in l.iter().rev() {
                    res = x.ite(y, &res);
                }
                res = c.ite(&t, &res);
                res
            }
            Expr::Forall(p, e, _) => {
                let exprs = combine_all(self.problem, p, e);
                let mut v = Vec::new();
                for e in exprs {
                    let e = e.type_inference(self.problem);
                    let e = self.to_bool(&e);
                    v.push(e);
                }
                z3::ast::Bool::and(self.ctx, &v.iter().collect::<Vec<_>>())
            }
            Expr::Exists(p, e, _) => {
                let exprs = combine_all(self.problem, p, e);
                let mut v = Vec::new();
                for e in exprs {
                    let e = e.type_inference(self.problem);
                    let e = self.to_bool(&e);
                    v.push(e);
                }
                z3::ast::Bool::or(self.ctx, &v.iter().collect::<Vec<_>>())
            }
            _ => panic!("to_bool {:?}", expr),
        }
    }

    pub fn to_int(&self, expr: &Expr) -> z3::ast::Int<'a> {
        match expr {
            Expr::IntValue(value, _) => z3::ast::Int::from_i64(self.ctx, *value as i64),
            Expr::Prefix(op, e, _) => match op {
                PreOp::Minus => {
                    let e = self.to_int(&e);
                    e.unary_minus()
                }
                _ => panic!(),
            },
            Expr::Binary(l, op, r, _) => {
                if l.typ(self.problem).is_integer() {
                    let l = self.to_int(l);
                    let r = self.to_int(r);
                    match op {
                        BinOp::Add => z3::ast::Int::add(self.ctx, &[&l, &r]),
                        BinOp::Sub => z3::ast::Int::sub(self.ctx, &[&l, &r]),
                        BinOp::Mul => z3::ast::Int::mul(self.ctx, &[&l, &r]),
                        _ => panic!(),
                    }
                } else {
                    panic!()
                }
            }
            Expr::Variable(id, _) => self.int_variable(*id).clone(),
            Expr::FunctionCall(id, parameters, _) => {
                self.fun_call(*id, parameters).as_int().unwrap()
            }
            Expr::StrucAttribute(e, id, _) => {
                let fun = self.struc_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_int().unwrap()
            }
            Expr::StrucMetCall(e, id, parameters, _) => {
                self.struc_met_call(e, *id, parameters).as_int().unwrap()
            }
            Expr::ClassAttribute(e, id, _) => {
                let fun = self.class_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_int().unwrap()
            }
            Expr::ClassMetCall(e, id, parameters, _) => {
                self.class_met_call(e, *id, parameters).as_int().unwrap()
            }
            Expr::AsInterval(e, min, max, _) => {
                let e = self.to_int(e);
                let min = z3::ast::Int::from_i64(self.ctx, *min as i64);
                let max = z3::ast::Int::from_i64(self.ctx, *max as i64);
                e.lt(&min).ite(&min, &e.gt(&max).ite(&max, &e))
            }
            Expr::IfThenElse(c, t, l, e, _) => {
                let c = self.to_bool(c);
                let t = self.to_int(t);
                let l = l
                    .iter()
                    .map(|(x, y)| (self.to_bool(x), self.to_int(y)))
                    .collect::<Vec<_>>();
                let e = self.to_int(e);
                let mut res = e;
                for (x, y) in l.iter().rev() {
                    res = x.ite(y, &res);
                }
                res = c.ite(&t, &res);
                res
            }
            _ => panic!("to_int {:?}", expr),
        }
    }

    pub fn to_real(&self, expr: &Expr) -> z3::ast::Real<'a> {
        match expr {
            Expr::RealValue(value, _) => z3::ast::Real::from_real(
                self.ctx,
                *value.numer().unwrap() as i32,
                *value.denom().unwrap() as i32,
            ),
            Expr::Prefix(op, e, _) => match op {
                PreOp::Minus => {
                    let e = self.to_real(&e);
                    e.unary_minus()
                }
                _ => panic!(),
            },
            Expr::Binary(l, op, r, _) => {
                let t = &l.typ(self.problem);
                if t.is_integer() {
                    let l = z3::ast::Real::from_int(&self.to_int(l));
                    let r = z3::ast::Real::from_int(&self.to_int(r));
                    match op {
                        BinOp::Div => l.div(&r),
                        _ => panic!(),
                    }
                } else if t.is_real() {
                    let l = self.to_real(l);
                    let r = self.to_real(r);
                    match op {
                        BinOp::Add => z3::ast::Real::add(self.ctx, &[&l, &r]),
                        BinOp::Sub => z3::ast::Real::sub(self.ctx, &[&l, &r]),
                        BinOp::Mul => z3::ast::Real::mul(self.ctx, &[&l, &r]),
                        BinOp::Div => l.div(&r),
                        _ => panic!(),
                    }
                } else {
                    panic!()
                }
            }
            Expr::Variable(id, _) => self.real_variable(*id).clone(),
            Expr::FunctionCall(id, parameters, _) => {
                self.fun_call(*id, parameters).as_real().unwrap()
            }
            Expr::StrucAttribute(e, id, _) => {
                let fun = self.struc_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_real().unwrap()
            }
            Expr::StrucMetCall(e, id, parameters, _) => {
                self.struc_met_call(e, *id, parameters).as_real().unwrap()
            }
            Expr::ClassAttribute(e, id, _) => {
                let fun = self.class_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_real().unwrap()
            }
            Expr::ClassMetCall(e, id, parameters, _) => {
                self.class_met_call(e, *id, parameters).as_real().unwrap()
            }
            Expr::IfThenElse(c, t, l, e, _) => {
                let c = self.to_bool(c);
                let t = self.to_real(t);
                let l = l
                    .iter()
                    .map(|(x, y)| (self.to_bool(x), self.to_real(y)))
                    .collect::<Vec<_>>();
                let e = self.to_real(e);
                let mut res = e;
                for (x, y) in l.iter().rev() {
                    res = x.ite(y, &res);
                }
                res = c.ite(&t, &res);
                res
            }
            _ => panic!("to_real {:?}", expr),
        }
    }

    pub fn to_datatype(&self, expr: &Expr) -> z3::ast::Datatype<'a> {
        match expr {
            Expr::FunctionCall(id, parameters, _) => {
                self.fun_call(*id, parameters).as_datatype().unwrap()
            }
            Expr::Instance(id, _) => {
                let inst = self.problem.get(*id).unwrap();
                if inst.typ().is_structure() {
                    self.struc_instance(*id).clone()
                } else if inst.typ().is_class() {
                    self.class_object(inst)
                } else {
                    panic!()
                }
            }
            Expr::Variable(id, _) => self.datatype_variable(*id).clone(),
            Expr::StrucAttribute(e, id, _) => {
                let fun = self.struc_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_datatype().unwrap()
            }
            Expr::StrucMetCall(e, id, parameters, _) => self
                .struc_met_call(e, *id, parameters)
                .as_datatype()
                .unwrap(),
            Expr::ClassAttribute(e, id, _) => {
                let fun = self.class_attributes.get(&id).unwrap();
                let e = self.to_datatype(e);
                fun.apply(&[&e]).as_datatype().unwrap()
            }
            Expr::ClassMetCall(e, id, parameters, _) => self
                .class_met_call(e, *id, parameters)
                .as_datatype()
                .unwrap(),
            Expr::AsClass(e, id) => match e.typ(self.problem) {
                Type::Class(c_id) => self.cast(self.to_datatype(e), c_id, *id),
                _ => panic!(),
            },
            Expr::IfThenElse(c, t, l, e, _) => {
                let c = self.to_bool(c);
                let t = self.to_datatype(t);
                let l = l
                    .iter()
                    .map(|(x, y)| (self.to_bool(x), self.to_datatype(y)))
                    .collect::<Vec<_>>();
                let e = self.to_datatype(e);
                let mut res = e;
                for (x, y) in l.iter().rev() {
                    res = x.ite(y, &res);
                }
                res = c.ite(&t, &res);
                res
            }
            _ => panic!("to_datatype {:?}", expr),
        }
    }

    fn cast(
        &self,
        e: z3::ast::Datatype<'a>,
        e_class: ClassId,
        class_id: ClassId,
    ) -> z3::ast::Datatype<'a> {
        if e_class == class_id {
            e
        } else {
            let super_id = self.problem.get(e_class).unwrap().super_class().unwrap();
            let e = self.object(e, e_class, super_id);
            self.cast(e, super_id, class_id)
        }
    }

    fn object(
        &self,
        e: z3::ast::Datatype<'a>,
        e_class: ClassId,
        target_class: ClassId,
    ) -> z3::ast::Datatype<'a> {
        let constructor_id = self
            .class_sort_variants
            .get(&target_class)
            .unwrap()
            .get(&e_class)
            .unwrap();
        let variant = &self
            .class_datatype_sort
            .get(&target_class)
            .unwrap()
            .variants[*constructor_id];
        variant.constructor.apply(&[&e]).as_datatype().unwrap()
    }

    //------------------------- -------------------------

    pub fn init(&mut self) {
        // Declare
        self.declare_structures();
        self.declare_classes();
        self.declare_variables();
        self.declare_functions();
        // Define
        self.define_structures();
        self.define_classes();
        self.define_variables();
        self.define_functions();
        // Constraint
        self.add_constraints();
        // Search
        self.add_search();
    }

    //------------------------- To Entry -------------------------

    pub fn solver_to_entry(&self) -> d_stuff::Entry {
        d_stuff::Entry::new(
            d_stuff::Status::Info,
            d_stuff::Text::new(
                "SMT Problem",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![d_stuff::Message::new(
                None,
                d_stuff::Text::new(
                    format!("{}", self.solver),
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                ),
            )],
        )
    }

    pub fn model_to_entry(&self) -> d_stuff::Entry {
        d_stuff::Entry::new(
            d_stuff::Status::Info,
            d_stuff::Text::new(
                "SMT Model",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![d_stuff::Message::new(
                None,
                d_stuff::Text::new(
                    format!("{}", self.solver.get_model().unwrap()),
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                ),
            )],
        )
    }
}
