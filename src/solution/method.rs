use super::*;
use crate::combine::Combine;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Method Value --------------------------------------------------

pub struct MethodValue {
    id: MethodId,
    calls: Vec<CallValue>,
}

impl MethodValue {
    pub fn new(smt: &Smt, model: &z3::Model, instance: InstanceId, method: &Method) -> Self {
        let mut calls = Vec::new();
        //
        let params_all = method
            .parameters()
            .iter()
            .map(|p| p.typ().all(smt.problem()))
            .collect();
        let mut combine = Combine::new(params_all);
        //
        loop {
            let inst = Expr::Instance(instance, None);
            let values = combine.values();
            let call = Expr::MethodCall(Box::new(inst), method.id(), values.clone(), None);
            //
            let value = CallValue::new(smt, model, values, &call);
            calls.push(value);
            //
            if !combine.step() {
                break;
            }
        }

        Self {
            id: method.id(),
            calls,
        }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for MethodValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut meth = problem.get(self.id).unwrap().clone();
        meth.clear_expr();
        let mut s = format!("{} {{\n", meth.to_lang(problem));
        for v in self.calls.iter() {
            s.push_str(&format!("        {}\n", v.to_lang(problem)));
        }
        s.push_str("        }");
        s
    }
}
