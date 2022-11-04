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

    //---------- Entry ----------

    pub fn entries(&self) -> Entries {
        let mut v = vec![];
        // TODO
        Entries::new(v)
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

    pub fn resolve(&mut self, entries: &Entries) -> Result<(), Error> {
        for x in self.attributes.iter_mut() {
            x.resolve(&entries)?;
        }
        for x in self.methods.iter_mut() {
            x.resolve(&entries)?;
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
