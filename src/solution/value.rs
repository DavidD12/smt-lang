use crate::problem::{Expr, GetFromId, InstanceId, Named, ToLang};
use crate::solve::Smt;
use fraction::Fraction;

pub enum Value {
    Instance(InstanceId),
    Bool(bool),
    Integer(isize),
    Real(Fraction),
}

impl Value {
    pub fn new(smt: &Smt, model: &z3::Model, expr: &Expr) -> Self {
        let t = &expr.typ(smt.problem());
        let expr = &expr.type_inference(smt.problem());
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
        } else if t.is_structure() {
            let value = model
                .eval(&smt.to_datatype(expr), true)
                .unwrap()
                .to_string();
            let instance = smt.problem().find_instance(&value).unwrap();
            Value::Instance(instance.id())
        } else if t.is_class() {
            let value = model
                .eval(&smt.to_datatype(expr), true)
                .unwrap()
                .to_string();
            let value = value.split(' ').last().unwrap().replace(")", "");
            let instance = smt.problem().find_instance(&value).unwrap();
            Value::Instance(instance.id())
        } else {
            panic!()
        }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for Value {
    fn to_lang(&self, problem: &crate::problem::Problem) -> String {
        match self {
            Value::Instance(id) => problem.get(*id).unwrap().name().to_string(),
            Value::Bool(value) => value.to_string(),
            Value::Integer(value) => value.to_string(),
            Value::Real(value) => value.to_string(),
        }
    }
}
