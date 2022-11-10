use super::*;
use crate::parser::Position;

pub enum StructureElement {
    Attribute(Attribute<StructureId>),
    Method(Method<StructureId>),
}

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct StructureId(pub usize);

impl Id for StructureId {
    fn empty() -> Self {
        Self(0)
    }
}

//------------------------- Structure -------------------------

#[derive(Clone)]
pub struct Structure {
    id: StructureId,
    name: String,
    typ: Type,
    attributes: Vec<Attribute<StructureId>>,
    methods: Vec<Method<StructureId>>,
    position: Option<Position>,
}

impl Structure {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = StructureId::empty();
        let name = name.into();
        Self {
            id,
            name,
            typ: Type::Structure(id),
            attributes: vec![],
            methods: vec![],
            position,
        }
    }

    //---------- Attribute ----------

    pub fn add_attribute(
        &mut self,
        mut attribute: Attribute<StructureId>,
    ) -> AttributeId<StructureId> {
        let id = AttributeId(self.id(), self.attributes.len());
        attribute.set_id(id);
        self.attributes.push(attribute);
        id
    }

    pub fn get_attribute(&self, id: AttributeId<StructureId>) -> Option<&Attribute<StructureId>> {
        let AttributeId(structure_id, n) = id;
        if self.id != structure_id {
            None
        } else {
            self.attributes.get(n)
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute<StructureId>> {
        &self.attributes
    }

    pub fn find_attribute(&self, name: &str) -> Option<&Attribute<StructureId>> {
        self.attributes.iter().find(|x| x.name() == name)
    }

    //---------- Method ----------

    pub fn add_method(&mut self, mut method: Method<StructureId>) -> MethodId<StructureId> {
        let id = MethodId(self.id(), self.methods.len());
        method.set_id(id);
        self.methods.push(method);
        id
    }

    pub fn get_method(&self, id: MethodId<StructureId>) -> Option<&Method<StructureId>> {
        let MethodId(structure_id, n) = id;
        if self.id != structure_id {
            None
        } else {
            self.methods.get(n)
        }
    }

    pub fn methods(&self) -> &Vec<Method<StructureId>> {
        &self.methods
    }

    pub fn find_method(&self, name: &str) -> Option<&Method<StructureId>> {
        self.methods.iter().find(|x| x.name() == name)
    }

    //---------- Instance ----------

    pub fn instances(&self, problem: &Problem) -> Vec<InstanceId> {
        let mut v = Vec::new();
        for inst in problem.instances().iter() {
            match inst.typ() {
                Type::Structure(id) => {
                    if *id == self.id() {
                        v.push(inst.id());
                    }
                }
                _ => {}
            }
        }
        v
    }

    pub fn is_empty(&self, problem: &Problem) -> bool {
        self.instances(problem).is_empty()
    }

    //---------- Duplicate ----------

    pub fn local_naming(&self) -> Vec<Naming> {
        let mut v = vec![];
        v.extend(self.attributes.iter().map(|x| x.naming()));
        v.extend(self.methods.iter().map(|x| x.naming()));
        v
    }

    pub fn duplicate(&self) -> Result<(), Error> {
        check_duplicate(self.local_naming())?;
        for x in self.methods.iter() {
            x.duplicate()?;
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_expr(&self, problem: &Problem, entries: &Entries) -> Result<Structure, Error> {
        // Attribute
        let mut attributes = Vec::new();
        for x in self.attributes.iter() {
            let a = x.resolve_expr(problem, &entries)?;
            attributes.push(a);
        }
        // Methods
        let mut methods = Vec::new();
        for x in self.methods.iter() {
            let m = x.resolve_expr(problem, &entries)?;
            methods.push(m);
        }
        //
        Ok(Structure {
            id: self.id,
            name: self.name.clone(),
            typ: self.typ.clone(),
            attributes,
            methods,
            position: self.position.clone(),
        })
    }

    //---------- Parameter Size ----------

    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.attributes.iter() {
            x.check_parameter_size(problem)?;
        }
        for x in self.methods.iter() {
            x.check_parameter_size(problem)?;
        }
        Ok(())
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.methods.iter() {
            x.check_bounded(problem)?;
        }
        Ok(())
    }

    //---------- Typing ----------

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.attributes.iter() {
            x.check_type(problem)?;
        }
        for x in self.methods.iter() {
            x.check_type(problem)?;
        }
        Ok(())
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Structure {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<StructureId> for Structure {
    fn id(&self) -> StructureId {
        self.id
    }

    fn set_id(&mut self, id: StructureId) {
        self.id = id;
        self.typ = Type::Structure(id);
        for (n, x) in self.attributes.iter_mut().enumerate() {
            let id = AttributeId(id, n);
            x.set_id(id);
        }
        for (n, x) in self.methods.iter_mut().enumerate() {
            let id = MethodId(id, n);
            x.set_id(id);
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Structure {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("struct {} {{\n", self.name());
        // Attribute
        for x in self.attributes.iter() {
            s.push_str(&format!("{}\n", &x.to_lang(problem)));
        }
        // Method
        for x in self.methods.iter() {
            s.push_str(&format!("{}", &x.to_lang(problem)));
            s.push_str(&format!("// {:?}\n", x.id()));
        }
        //
        s.push_str("}\n");
        s
    }
}

//------------------------- With Type -------------------------

impl WithType for Structure {
    fn typ(&self) -> &Type {
        &self.typ
    }

    fn set_type(&mut self, _: Type) {}

    fn resolve_type_children(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        for x in self.attributes.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.methods.iter_mut() {
            x.resolve_type(&entries)?;
        }
        Ok(())
    }

    fn check_interval_children(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.attributes.iter() {
            x.check_interval(problem)?;
        }
        for x in self.methods.iter() {
            x.check_interval(problem)?;
        }
        Ok(())
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<AttributeId<StructureId>, Attribute<StructureId>> for Structure {
    fn get(&self, id: AttributeId<StructureId>) -> Option<&Attribute<StructureId>> {
        self.get_attribute(id)
    }
}

impl GetFromId<MethodId<StructureId>, Method<StructureId>> for Structure {
    fn get(&self, id: MethodId<StructureId>) -> Option<&Method<StructureId>> {
        self.get_method(id)
    }
}
