use super::*;
use crate::combine::Combine;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Function --------------------------------------------------

pub struct FunctionValue {
    calls: Vec<CallValue>,
}

impl FunctionValue {
    pub fn new(smt: &Smt, model: &z3::Model, function: &Function) -> Self {
        let mut calls = Vec::new();
        //
        let params_all = function
            .parameters()
            .iter()
            .map(|p| p.typ().all(smt.problem()))
            .collect();
        let mut combine = Combine::new(params_all);
        //
        loop {
            let values = combine.values();
            let call = Expr::FunctionCall(function.id(), values.clone(), None);
            //
            let value = CallValue::new(smt, model, values, &call);
            calls.push(value);
            //
            if !combine.step() {
                break;
            }
        }

        Self { calls }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for FunctionValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = "{\n".to_string();
        for v in self.calls.iter() {
            s.push_str(&format!("{}\n", v.to_lang(problem)));
        }
        s.push_str("}");
        s
    }
}
