use super::*;
use crate::combine::Combine;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Value --------------------------------------------------

pub struct ApplyValue {
    parameters: Vec<Value>,
    value: Value,
}

impl ApplyValue {
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

impl ToLang for ApplyValue {
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

//-------------------------------------------------- Function --------------------------------------------------

pub struct FunctionValue {
    applies: Vec<ApplyValue>,
}

impl FunctionValue {
    pub fn new(smt: &Smt, model: &z3::Model, function: &Function) -> Self {
        let mut applies = Vec::new();
        //
        let params_all = function
            .parameters()
            .iter()
            .map(|p| p.typ().all())
            .collect();
        let mut combine = Combine::new(params_all);
        //
        loop {
            let values = combine.values();
            let call = Expr::FunctionCall(function.id(), values.clone(), None);
            //
            let value = ApplyValue::new(smt, model, values, &call);
            applies.push(value);
            //
            if !combine.step() {
                break;
            }
        }

        Self { applies }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for FunctionValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = "{\n".to_string();
        for v in self.applies.iter() {
            s.push_str(&format!("{}\n", v.to_lang(problem)));
        }
        s.push_str("}");
        s
    }
}
