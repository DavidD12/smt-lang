use super::*;
use crate::parser::Position;

pub enum StructureElement {
    Attribute(Attribute),
    Method(Method),
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
    attributes: Vec<Attribute>,
    methods: Vec<Method>,
    position: Option<Position>,
}

impl Structure {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = StructureId::empty();
        let name = name.into();
        Self {
            id,
            name,
            attributes: vec![],
            methods: vec![],
            position,
        }
    }

    //---------- Attribute ----------

    pub fn add_attribute(&mut self, mut attribute: Attribute) -> AttributeId {
        let id = AttributeId(self.id(), self.attributes.len());
        attribute.set_id(id);
        self.attributes.push(attribute);
        id
    }

    pub fn get_attribute(&self, id: AttributeId) -> Option<&Attribute> {
        let AttributeId(structure_id, n) = id;
        if self.id != structure_id {
            None
        } else {
            self.attributes.get(n)
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn find_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.iter().find(|x| x.name() == name)
    }

    //---------- Method ----------

    pub fn add_method(&mut self, mut method: Method) -> MethodId {
        let id = MethodId(self.id(), self.methods.len());
        method.set_id(id);
        self.methods.push(method);
        id
    }

    pub fn get_method(&self, id: MethodId) -> Option<&Method> {
        let MethodId(structure_id, n) = id;
        if self.id != structure_id {
            None
        } else {
            self.methods.get(n)
        }
    }

    pub fn methods(&self) -> &Vec<Method> {
        &self.methods
    }

    pub fn find_method(&self, name: &str) -> Option<&Method> {
        self.methods.iter().find(|x| x.name() == name)
    }

    //---------- Type ----------

    pub fn typ(&self) -> Type {
        Type::Structure(self.id)
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

    pub fn resolve_type(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        for x in self.attributes.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.methods.iter_mut() {
            x.resolve_type(&entries)?;
        }
        Ok(())
    }

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
            attributes,
            methods,
            position: self.position.clone(),
        })
    }

    //---------- Interval ----------

    pub fn check_interval(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.attributes.iter() {
            x.check_interval(problem)?;
        }
        for x in self.methods.iter() {
            x.check_interval(problem)?;
        }
        Ok(())
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

//------------------------- Named -------------------------

impl Named<StructureId> for Structure {
    fn id(&self) -> StructureId {
        self.id
    }

    fn set_id(&mut self, id: StructureId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> &Option<Position> {
        &self.position
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
            s.push_str(&format!("{}\n", &x.to_lang(problem)));
        }
        //
        s.push_str("}\n");
        s
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<AttributeId, Attribute> for Structure {
    fn get(&self, id: AttributeId) -> Option<&Attribute> {
        self.get_attribute(id)
    }
}

impl GetFromId<MethodId, Method> for Structure {
    fn get(&self, id: MethodId) -> Option<&Method> {
        self.get_method(id)
    }
}
