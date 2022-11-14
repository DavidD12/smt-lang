use super::*;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Attribute Value --------------------------------------------------

pub struct AttributeValue<T: Id> {
    id: AttributeId<T>,
    value: Value,
}

pub trait NewAttribute<T: Id> {
    fn new(smt: &Smt, model: &z3::Model, instance: InstanceId, attribute: AttributeId<T>) -> Self;
}

impl NewAttribute<StructureId> for AttributeValue<StructureId> {
    fn new(
        smt: &Smt,
        model: &z3::Model,
        instance: InstanceId,
        attribute: AttributeId<StructureId>,
    ) -> Self {
        let e = Expr::Instance(instance, None);
        let expr = Expr::StrucAttribute(Box::new(e), attribute, None);
        let value = Value::new(smt, model, &expr);
        Self {
            id: attribute,
            value,
        }
    }
}

impl ToLang for AttributeValue<StructureId> {
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

impl NewAttribute<ClassId> for AttributeValue<ClassId> {
    fn new(
        smt: &Smt,
        model: &z3::Model,
        instance: InstanceId,
        attribute: AttributeId<ClassId>,
    ) -> Self {
        let e = Expr::Instance(instance, None);
        let expr = Expr::ClassAttribute(Box::new(e), attribute, None);
        let value = Value::new(smt, model, &expr);
        Self {
            id: attribute,
            value,
        }
    }
}

impl ToLang for AttributeValue<ClassId> {
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
