use crate::problem::*;

impl Expr {
    pub fn typ(&self, problem: &Problem) -> Type {
        match self {
            Expr::BoolValue(_, _) => Type::Bool,
            Expr::IntValue(value, _) => Type::Interval(*value, *value),
            Expr::RealValue(_, _) => Type::Real,
            Expr::Unary(op, e, _) => match op {
                UnaryOp::Not => Type::Bool,
                UnaryOp::Minus => match e.typ(problem) {
                    Type::Int => Type::Int,
                    Type::Real => Type::Real,
                    Type::Interval(min, max) => Type::Interval(-max, -min),
                    _ => Type::Undefined,
                },
            },
            Expr::Binary(left, op, right, _) => match op {
                BinOp::Eq => Type::Bool,
                BinOp::Ne => Type::Bool,
                BinOp::Lt => Type::Bool,
                BinOp::Le => Type::Bool,
                BinOp::Ge => Type::Bool,
                BinOp::Gt => Type::Bool,
                BinOp::And => Type::Bool,
                BinOp::Or => Type::Bool,
                BinOp::Implies => Type::Bool,
                //
                BinOp::Add => match (left.typ(problem), right.typ(problem)) {
                    (Type::Int, _) => Type::Int,
                    (Type::Real, _) => Type::Real,
                    (Type::Interval(_, _), Type::Int) => Type::Int,
                    (Type::Interval(min1, max1), Type::Interval(min2, max2)) => {
                        Type::Interval(min1 + min2, max1 + max2)
                    }
                    _ => Type::Undefined,
                },
                BinOp::Sub => match (left.typ(problem), right.typ(problem)) {
                    (Type::Int, _) => Type::Int,
                    (Type::Real, _) => Type::Real,
                    (Type::Interval(_, _), Type::Int) => Type::Int,
                    (Type::Interval(min1, max1), Type::Interval(min2, max2)) => {
                        Type::Interval(min1 - min2, max1 - max2)
                    }
                    _ => Type::Undefined,
                },
                BinOp::Mul => match (left.typ(problem), right.typ(problem)) {
                    (Type::Int, _) => Type::Int,
                    (Type::Real, _) => Type::Real,
                    (Type::Interval(_, _), Type::Int) => Type::Int,
                    (Type::Interval(min1, max1), Type::Interval(min2, max2)) => {
                        Type::Interval(min1 * min2, max1 * max2)
                    }
                    _ => Type::Undefined,
                },
                BinOp::Div => Type::Real,
            },
            Expr::FunctionCall(id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::Instance(id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::Variable(id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::Parameter(p) => p.typ().clone(),
            Expr::StrucSelf(id, _) => Type::Structure(*id),
            Expr::StrucAttribute(_, id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::StrucMetCall(_, id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::ClassSelf(id, _) => Type::Class(*id),
            Expr::ClassAttribute(_, id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::ClassMetCall(_, id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::AsClass(_, id) => Type::Class(*id),
            Expr::AsInterval(_, min, max, _) => Type::Interval(*min, *max),
            Expr::AsInt(_, _) => Type::Int,
            Expr::AsReal(_, _) => Type::Real,
            Expr::IfThenElse(_, t, l, e, _) => {
                let mut res = t.typ(problem);
                for (_, x) in l.iter() {
                    res = res.common_type(problem, &x.typ(problem));
                }
                res = res.common_type(problem, &e.typ(problem));
                res
            }
            Expr::Quantifier(op, _, e, _) => match op {
                QtOp::Forall => Type::Bool,
                QtOp::Exists => Type::Bool,
                QtOp::Sum => match e.typ(problem) {
                    t @ Type::Real => t,
                    t @ Type::Int => t,
                    Type::Interval(_, _) => Type::Int,
                    _ => Type::Undefined,
                },
                QtOp::Prod => match e.typ(problem) {
                    t @ Type::Real => t,
                    t @ Type::Int => t,
                    Type::Interval(_, _) => Type::Int,
                    _ => Type::Undefined,
                },
            },
            Expr::Unresolved(_, _) => Type::Undefined,
            Expr::UnresolvedFunCall(_, _, _) => Type::Undefined,
            Expr::UnresolvedAttribute(_, _, _) => Type::Undefined,
            Expr::UnresolvedMethCall(_, _, _, _) => Type::Undefined,
        }
    }
}
