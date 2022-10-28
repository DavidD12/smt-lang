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
            constraints: HashMap::new(),
        }
    }

    //------------------------- Variable -------------------------

    pub fn problem(&self) -> &Problem {
        self.problem
    }

    //------------------------- Variable -------------------------

    fn add_variable(&mut self, variable: &Variable) {
        match variable.typ() {
            Type::Bool => {
                let v = z3::ast::Bool::new_const(self.ctx, variable.name());
                if let Some(e) = variable.expr() {
                    let e = self.to_bool(e);
                    self.solver.assert(&v._eq(&e));
                }
                self.bool_variables.insert(variable.id(), v);
            }
            Type::Int => {
                let v = z3::ast::Int::new_const(self.ctx, variable.name());
                if let Some(e) = variable.expr() {
                    let e = self.to_int(e);
                    self.solver.assert(&v._eq(&e));
                }
                self.int_variables.insert(variable.id(), v);
            }
            Type::Real => {
                let v = z3::ast::Real::new_const(self.ctx, variable.name());
                if let Some(e) = variable.expr() {
                    let e = self.to_real(e);
                    self.solver.assert(&v._eq(&e));
                }
                self.real_variables.insert(variable.id(), v);
            }
            Type::Undefined => panic!(),
        }
    }

    fn add_variables(&mut self) {
        for v in self.problem.variables().iter() {
            self.add_variable(v);
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

    //------------------------- Constraint -------------------------

    fn add_constraint(&mut self, constraint: &Constraint) {
        let c = z3::ast::Bool::new_const(self.ctx, constraint.name());
        let e = self.to_bool(constraint.expr());
        self.solver.assert(&c._eq(&e));
        self.solver.assert(&c);
        self.constraints.insert(constraint.id(), c);
    }

    fn add_constraints(&mut self) {
        for c in self.problem.constraints().iter() {
            self.add_constraint(c);
        }
    }

    pub fn constraint(&self, id: ConstraintId) -> &z3::ast::Bool<'a> {
        self.constraints.get(&id).unwrap()
    }

    //------------------------- Expr -------------------------

    fn to_bool(&self, expr: &Expr) -> z3::ast::Bool<'a> {
        match expr {
            Expr::BoolValue(value, _) => z3::ast::Bool::from_bool(self.ctx, *value),
            Expr::PreExpr(op, e, _) => match op {
                PreOp::Not => {
                    let e = self.to_bool(e);
                    z3::ast::Bool::not(&e)
                }
                _ => panic!(),
            },
            Expr::BinExpr(l, op, r, _) => match l.typ(self.problem) {
                Type::Bool => {
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
                }
                Type::Int => {
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
                }
                Type::Real => {
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
                }
                _ => panic!(),
            },
            Expr::Variable(id, _) => self.bool_variable(*id).clone(),
            _ => panic!(),
        }
    }

    fn to_int(&self, expr: &Expr) -> z3::ast::Int<'a> {
        match expr {
            Expr::IntValue(value, _) => z3::ast::Int::from_i64(self.ctx, *value as i64),
            Expr::PreExpr(op, e, _) => match op {
                PreOp::Minus => {
                    let e = self.to_int(&e);
                    e.unary_minus()
                }
                _ => panic!(),
            },
            Expr::BinExpr(l, op, r, _) => match l.typ(self.problem) {
                Type::Int => {
                    let l = self.to_int(l);
                    let r = self.to_int(r);
                    match op {
                        BinOp::Add => z3::ast::Int::add(self.ctx, &[&l, &r]),
                        BinOp::Sub => z3::ast::Int::sub(self.ctx, &[&l, &r]),
                        BinOp::Mul => z3::ast::Int::mul(self.ctx, &[&l, &r]),
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            },
            Expr::Variable(id, _) => self.int_variable(*id).clone(),
            _ => panic!(),
        }
    }

    fn to_real(&self, expr: &Expr) -> z3::ast::Real<'a> {
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
            Expr::BinExpr(l, op, r, _) => match l.typ(self.problem) {
                Type::Int => {
                    let l = z3::ast::Real::from_int(&self.to_int(l));
                    let r = z3::ast::Real::from_int(&self.to_int(r));
                    match op {
                        BinOp::Div => l.div(&r),
                        _ => panic!(),
                    }
                }
                Type::Real => {
                    let l = self.to_real(l);
                    let r = self.to_real(r);
                    match op {
                        BinOp::Add => z3::ast::Real::add(self.ctx, &[&l, &r]),
                        BinOp::Sub => z3::ast::Real::sub(self.ctx, &[&l, &r]),
                        BinOp::Mul => z3::ast::Real::mul(self.ctx, &[&l, &r]),
                        BinOp::Div => l.div(&r),
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            },
            Expr::Variable(id, _) => self.real_variable(*id).clone(),
            _ => panic!(),
        }
    }

    //------------------------- -------------------------

    pub fn init(&mut self) {
        self.add_variables();
        self.add_constraints();
    }
}
