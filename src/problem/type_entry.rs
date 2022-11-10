use super::*;

//------------------------- TypeEntry Type -------------------------

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TypeEntryType {
    Structure(StructureId),
    Class(ClassId),
}

//------------------------- TypeEntry -------------------------

#[derive(Clone, Debug)]
pub struct TypeEntry {
    name: String,
    typ: TypeEntryType,
}

impl TypeEntry {
    pub fn new(name: String, typ: TypeEntryType) -> Self {
        Self { name, typ }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typ(&self) -> TypeEntryType {
        self.typ
    }
}

impl FromId<StructureId> for TypeEntry {
    fn from_id(problem: &Problem, id: StructureId) -> Self {
        let name = problem.get(id).unwrap().name().into();
        let typ = TypeEntryType::Structure(id);
        Self { name, typ }
    }
}

impl FromId<ClassId> for TypeEntry {
    fn from_id(problem: &Problem, id: ClassId) -> Self {
        let name = problem.get(id).unwrap().name().into();
        let typ = TypeEntryType::Class(id);
        Self { name, typ }
    }
}

//------------------------- TypeEntries -------------------------

#[derive(Clone, Debug)]
pub struct TypeEntries(Vec<TypeEntry>);

impl TypeEntries {
    pub fn new(entries: Vec<TypeEntry>) -> Self {
        TypeEntries(entries)
    }

    fn entries(&self) -> &Vec<TypeEntry> {
        let TypeEntries(entries) = self;
        entries
    }

    pub fn add(&self, entry: TypeEntry) -> TypeEntries {
        let mut v = self.entries().clone();
        v.push(entry);
        TypeEntries(v)
    }

    pub fn get(&self, name: &str) -> Option<TypeEntry> {
        for e in self.entries().iter().rev() {
            if e.name() == name {
                return Some(e.clone());
            }
        }
        None
    }
}
