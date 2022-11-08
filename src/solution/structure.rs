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

//-------------------------------------------------- Method Value --------------------------------------------------

//-------------------------------------------------- Instance Value --------------------------------------------------

pub struct InstanceValue {
    id: InstanceId,
    attributes: Vec<AttributeValue>,
}

impl InstanceValue {
    pub fn new(smt: &Smt, model: &z3::Model, strcuture: StructureId, instance: InstanceId) -> Self {
        let structure = smt.problem().get(strcuture).unwrap();
        let mut attributes = Vec::new();
        for attribute in structure.attributes().iter() {
            let av = AttributeValue::new(smt, model, instance, attribute.id());
            attributes.push(av);
        }
        Self {
            id: instance,
            attributes,
        }
    }
}

impl ToLang for InstanceValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let instance = problem.get(self.id).unwrap();
        let mut s = format!("    inst {} {{\n", instance.name());
        for attribute in self.attributes.iter() {
            s.push_str(&format!("        {}\n", attribute.to_lang(problem)));
        }
        s.push_str("    }\n");
        s
    }
}

//-------------------------------------------------- Structure Value --------------------------------------------------

pub struct StructureValue {
    id: StructureId,
    instances: Vec<InstanceValue>,
}

impl StructureValue {
    pub fn new(smt: &Smt, model: &z3::Model, structure: StructureId) -> Self {
        let mut instances = Vec::new();
        for instance in smt.problem().structure_instances(structure) {
            instances.push(InstanceValue::new(smt, model, structure, instance));
        }
        Self {
            id: structure,
            instances,
        }
    }
}

impl ToLang for StructureValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let structure = problem.get(self.id).unwrap();
        let mut s = format!("struct {} {{\n", structure.name());
        for instance in self.instances.iter() {
            s.push_str(&instance.to_lang(problem));
        }
        s.push_str("}\n");
        s
    }
}
