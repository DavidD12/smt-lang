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

impl std::fmt::Display for ApplyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    (")?;
        if let Some((first, others)) = self.parameters.split_first() {
            write!(f, "{}", first)?;
            for p in others.iter() {
                write!(f, ", {}", p)?;
            }
        }
        write!(f, ") -> {}", self.value)
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

impl std::fmt::Display for FunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for v in self.applies.iter() {
            write!(f, "{}\n", v)?;
        }
        write!(f, "}}")
    }
}
