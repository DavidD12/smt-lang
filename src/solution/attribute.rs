use super::*;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Attribute Value --------------------------------------------------

pub struct AttributeValue {
    id: AttributeId,
    value: Value,
}

impl AttributeValue {
    pub fn new(smt: &Smt, model: &z3::Model, instance: InstanceId, attribute: AttributeId) -> Self {
        let e = Expr::Instance(instance, None);
        let expr = Expr::Attribute(Box::new(e), attribute, None);
        let value = Value::new(smt, model, &expr);
        Self {
            id: attribute,
            value,
        }
    }
}

impl ToLang for AttributeValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let attribute = problem.get(self.id).unwrap();
        format!(
            "{}: {} = {}",
            attribute.name(),
            attribute.typ().to_lang(problem),
            self.value.to_lang(problem)
        )
    }
}
