use super::*;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Instance Value --------------------------------------------------

pub struct InstanceValue {
    id: InstanceId,
    attributes: Vec<AttributeValue>,
    methods: Vec<MethodValue>,
}

impl InstanceValue {
    pub fn new(smt: &Smt, model: &z3::Model, strcuture: StructureId, instance: InstanceId) -> Self {
        let structure = smt.problem().get(strcuture).unwrap();
        // Attributes
        let mut attributes = Vec::new();
        for attribute in structure.attributes().iter() {
            let av = AttributeValue::new(smt, model, instance, attribute.id());
            attributes.push(av);
        }
        // Methods
        let mut methods = Vec::new();
        for method in structure.methods().iter() {
            let mv = MethodValue::new(smt, model, instance, method);
            methods.push(mv);
        }
        //
        Self {
            id: instance,
            attributes,
            methods,
        }
    }
}

impl ToLang for InstanceValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let instance = problem.get(self.id).unwrap();
        let mut s = format!("    inst {} {{\n", instance.name());
        // Attribute
        for attribute in self.attributes.iter() {
            s.push_str(&format!("        {}\n", attribute.to_lang(problem)));
        }
        // Method
        for method in self.methods.iter() {
            s.push_str(&format!("    {}\n", method.to_lang(problem)));
        }
        //
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
