use super::*;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Call Value --------------------------------------------------

pub struct CallValue {
    parameters: Vec<Value>,
    value: Value,
}

impl CallValue {
    pub fn new(smt: &Smt, model: &z3::Model, parameters: Vec<Expr>, value: &Expr) -> Self {
        let parameters = parameters
            .iter()
            .map(|p| Value::new(smt, model, p))
            .collect();
        let value = Value::new(smt, model, value);
        Self { parameters, value }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for CallValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = "    (".to_string();
        if let Some((first, others)) = self.parameters.split_first() {
            s.push_str(&format!("{}", first.to_lang(problem)));
            for p in others.iter() {
                s.push_str(&format!(", {}", p.to_lang(problem)));
            }
        }
        s.push_str(&format!(") -> {}", self.value.to_lang(problem)));
        s
    }
}
