use crate::problem::Expr;
use crate::solve::Smt;
use fraction::Fraction;

pub enum Value {
    Bool(bool),
    Integer(isize),
    Real(Fraction),
}

impl Value {
    pub fn new(smt: &Smt, model: &z3::Model, expr: &Expr) -> Self {
        let t = &expr.typ(smt.problem());
        if t.is_bool() {
            let value = model
                .eval(&smt.to_bool(expr), true)
                .unwrap()
                .as_bool()
                .unwrap();
            Value::Bool(value)
        } else if t.is_integer() {
            let value = model
                .eval(&smt.to_int(expr), true)
                .unwrap()
                .as_i64()
                .unwrap();

            Value::Integer(value as isize)
        } else if t.is_real() {
            let (num, den) = model
                .eval(&smt.to_real(expr), true)
                .unwrap()
                .as_real()
                .unwrap();
            let value = if num >= 0 {
                Fraction::new_generic(fraction::Sign::Plus, num, den).unwrap()
            } else {
                Fraction::new_generic(fraction::Sign::Minus, -num, den).unwrap()
            };
            Value::Real(value)
        } else {
            panic!()
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::Real(value) => write!(f, "{}", value),
        }
    }
}
