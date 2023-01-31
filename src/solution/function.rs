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
        //
        let fun = smt.function(function.id());
        let i = z3::ast::Int::fresh_const(smt.ctx(), "i");
        let j = z3::ast::Int::fresh_const(smt.ctx(), "j");
        let app = fun.apply(&[&i, &j]);
        let e = model.eval(&app, false).unwrap();
        println!("{:?}", e);
        //
        let i = z3::ast::Int::from_i64(smt.ctx(), 1);
        let app = fun.apply(&[&i, &j]);
        let e = model.eval(&app, false).unwrap();
        println!("{:?}", e);
        //
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
