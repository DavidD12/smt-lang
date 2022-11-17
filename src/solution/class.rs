use super::*;
use crate::problem::*;
use crate::solve::Smt;

//-------------------------------------------------- Instance Value --------------------------------------------------

pub struct ClassInstanceValue {
    id: InstanceId,
    attributes: Vec<AttributeValue<ClassId>>,
    methods: Vec<MethodValue<ClassId>>,
}

impl ClassInstanceValue {
    pub fn new(smt: &Smt, model: &z3::Model, class: ClassId, instance: InstanceId) -> Self {
        let class = smt.problem().get(class).unwrap();
        // Attributes
        let mut attributes = Vec::new();
        for id in class.all_attributes(smt.problem()).iter() {
            let av = AttributeValue::new(smt, model, instance, *id);
            attributes.push(av);
        }
        // Methods
        let mut methods = Vec::new();
        for id in class.all_methods(smt.problem()).iter() {
            let method = smt.problem().get(*id).unwrap();
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

impl ToLang for ClassInstanceValue {
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

//-------------------------------------------------- Class Value --------------------------------------------------

pub struct ClassValue {
    id: ClassId,
    instances: Vec<ClassInstanceValue>,
}

impl ClassValue {
    pub fn new(smt: &Smt, model: &z3::Model, class: ClassId) -> Self {
        let mut instances = Vec::new();
        for instance in smt.problem().get(class).unwrap().instances(smt.problem()) {
            instances.push(ClassInstanceValue::new(smt, model, class, instance));
        }
        Self {
            id: class,
            instances,
        }
    }
}

impl ToLang for ClassValue {
    fn to_lang(&self, problem: &Problem) -> String {
        let structure = problem.get(self.id).unwrap();
        let mut s = format!("class {} {{\n", structure.name());
        for instance in self.instances.iter() {
            s.push_str(&instance.to_lang(problem));
        }
        s.push_str("}\n");
        s
    }
}
