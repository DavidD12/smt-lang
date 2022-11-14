use super::*;
use crate::combine::Combine;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Method Value --------------------------------------------------

pub struct MethodValue<T: Id> {
    id: MethodId<T>,
    calls: Vec<CallValue>,
}

pub trait NewMethod<T: Id> {
    fn new(smt: &Smt, model: &z3::Model, instance: InstanceId, method: &Method<T>) -> Self;
}

impl NewMethod<StructureId> for MethodValue<StructureId> {
    fn new(
        smt: &Smt,
        model: &z3::Model,
        instance: InstanceId,
        method: &Method<StructureId>,
    ) -> Self {
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
            let call = Expr::StrucMetCall(Box::new(inst), method.id(), values.clone(), None);
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

impl ToLang for MethodValue<StructureId> {
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

impl NewMethod<ClassId> for MethodValue<ClassId> {
    fn new(smt: &Smt, model: &z3::Model, instance: InstanceId, method: &Method<ClassId>) -> Self {
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
            let call = Expr::ClassMetCall(Box::new(inst), method.id(), values.clone(), None);
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

impl ToLang for MethodValue<ClassId> {
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
