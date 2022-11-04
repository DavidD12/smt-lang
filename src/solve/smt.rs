use crate::combine::Combine;
use crate::problem::*;
use std::collections::HashMap;
use z3::ast::Ast;

pub struct Smt<'a> {
    problem: &'a Problem,
    //
    _cfg: &'a z3::Config,
    ctx: &'a z3::Context,
    solver: &'a z3::Solver<'a>,
    // Variable
    bool_variables: HashMap<VariableId, z3::ast::Bool<'a>>,
    int_variables: HashMap<VariableId, z3::ast::Int<'a>>,
    real_variables: HashMap<VariableId, z3::ast::Real<'a>>,
    // Function
    functions: HashMap<FunctionId, z3::FuncDecl<'a>>,
    // Constraint
    constraints: HashMap<ConstraintId, z3::ast::Bool<'a>>,
}

impl<'a> Smt<'a> {
    pub fn new(
        problem: &'a Problem,
        cfg: &'a z3::Config,
        ctx: &'a z3::Context,
        solver: &'a z3::Solver,
    ) -> Self {
        Self {
            problem,
            _cfg: cfg,
            ctx,
            solver,
            bool_variables: HashMap::new(),
            int_variables: HashMap::new(),
            real_variables: HashMap::new(),
            functions: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    //------------------------- -------------------------

    pub fn problem(&self) -> &Problem {
        self.problem
    }

    //------------------------- Sort -------------------------

    fn to_sort(&self, typ: &Type) -> z3::Sort<'a> {
        match typ {
            Type::Bool => z3::Sort::bool(self.ctx),
            Type::Int => z3::Sort::int(self.ctx),
            Type::Real => z3::Sort::real(self.ctx),
            Type::Interval(_, _) => z3::Sort::int(self.ctx),
            Type::Function(_, t) => self.to_sort(t),
            Type::Undefined => panic!(),
        }
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
            Type::Interval(_, _) => {
                let v = z3::ast::Int::new_const(self.ctx, variable.name());
                self.int_variables.insert(variable.id(), v);
            }
            Type::Function(_, _) => panic!(),
            Type::Undefined => panic!(),
        }
    }

    fn define_variable(&mut self, variable: &Variable) {
        match variable.typ() {
            Type::Bool => {
                let v = self.bool_variable(variable.id());
                if let Some(e) = variable.expr() {
                    let e = self.to_bool(e);
                    self.solver.assert(&v._eq(&e));
                }
            }
            Type::Int => {
                let v = self.int_variable(variable.id());
                if let Some(e) = variable.expr() {
                    let e = self.to_int(e);
                    self.solver.assert(&v._eq(&e));
                }
            }
            Type::Real => {
                let v = self.real_variable(variable.id());
                if let Some(e) = variable.expr() {
                    let e = self.to_real(e);
                    self.solver.assert(&v._eq(&e));
                }
            }
            Type::Interval(min, max) => {
                let v = self.int_variable(variable.id());
                if let Some(e) = variable.expr() {
                    let e = self.to_int(e);
                    self.solver.assert(&v._eq(&e));
                }
                self.solver
                    .assert(&v.ge(&z3::ast::Int::from_i64(self.ctx, min as i64)));
                self.solver
                    .assert(&v.le(&z3::ast::Int::from_i64(self.ctx, max as i64)));
            }
            Type::Function(_, _) => panic!(),
            Type::Undefined => panic!(),
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

    //------------------------- Function -------------------------

    fn declare_function(&mut self, function: &Function) {
        let params = function
            .parameters()
            .iter()
            .map(|p| self.to_sort(&p.typ()))
            .collect::<Vec<_>>();
        let params = params.iter().collect::<Vec<_>>();
        let fun = z3::FuncDecl::new(
            self.ctx,
            function.name(),
            &params,
            &self.to_sort(&function.return_type()),
        );
        self.functions.insert(function.id(), fun);
    }

    fn define_function(&mut self, function: &Function) {
        if let Some(expr) = function.expr() {
            let params_all = function
                .parameters()
                .iter()
                .map(|p| p.typ().all())
                .collect();
            let param_expr = function
                .parameters()
                .iter()
                .map(|p| Expr::Parameter(p.id(), p.position().clone()))
                .collect::<Vec<_>>();
            let mut combine = Combine::new(params_all);
            loop {
                let values = combine.values();
                let all = param_expr
                    .clone()
                    .into_iter()
                    .zip(values.clone())
                    .collect::<Vec<_>>();
                let e = expr.substitute_all(all);
                let call = Expr::FunctionCall(function.id(), values, None);
                if function.return_type().is_bool() {
                    let call = self.to_bool(&call);
                    let e = self.to_bool(&e);
                    self.solver.assert(&call._eq(&e));
                } else if function.return_type().is_integer() {
                    let call = self.to_int(&call);
                    let e = self.to_int(&e);
                    self.solver.assert(&call._eq(&e));
                } else if function.return_type().is_real() {
                    let call = self.to_real(&call);
                    let e = self.to_real(&e);
                    self.solver.assert(&call._eq(&e));
                } else {
                    panic!()
                }
                if !combine.step() {
                    break;
                }
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
        let e = self.to_bool(constraint.expr());
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

    pub fn to_bool(&self, expr: &Expr) -> z3::ast::Bool<'a> {
        match expr {
            Expr::BoolValue(value, _) => z3::ast::Bool::from_bool(self.ctx, *value),
            Expr::PreExpr(op, e, _) => match op {
                PreOp::Not => {
                    let e = self.to_bool(e);
                    z3::ast::Bool::not(&e)
                }
                _ => panic!(),
            },
            Expr::BinExpr(l, op, r, _) => {
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
                } else {
                    panic!()
                }
            }
            Expr::Variable(id, _) => self.bool_variable(*id).clone(),
            Expr::FunctionCall(id, parameters, _) => {
                self.fun_call(*id, parameters).as_bool().unwrap()
            }
            _ => panic!("to_bool {:?}", expr),
        }
    }

    pub fn to_int(&self, expr: &Expr) -> z3::ast::Int<'a> {
        match expr {
            Expr::IntValue(value, _) => z3::ast::Int::from_i64(self.ctx, *value as i64),
            Expr::PreExpr(op, e, _) => match op {
                PreOp::Minus => {
                    let e = self.to_int(&e);
                    e.unary_minus()
                }
                _ => panic!(),
            },
            Expr::BinExpr(l, op, r, _) => {
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
            Expr::PreExpr(op, e, _) => match op {
                PreOp::Minus => {
                    let e = self.to_real(&e);
                    e.unary_minus()
                }
                _ => panic!(),
            },
            Expr::BinExpr(l, op, r, _) => {
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
            _ => panic!("to_real {:?}", expr),
        }
    }

    //------------------------- -------------------------

    pub fn init(&mut self) {
        // Declare
        self.declare_variables();
        self.declare_functions();
        // Define
        self.define_variables();
        self.define_functions();
        // Constraint
        self.add_constraints();
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
