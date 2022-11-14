use super::*;

//------------------------- Entry Type -------------------------

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EntryType {
    Instance(InstanceId),
    Variable(VariableId),
    //
    Parameter(Parameter),
    //
    StrucSelf(StructureId),
    //
    ClassSelf(ClassId),
}

//------------------------- Entry -------------------------

#[derive(Clone, Debug)]
pub struct Entry {
    name: String,
    typ: EntryType,
}

impl Entry {
    pub fn new(name: String, typ: EntryType) -> Self {
        Self { name, typ }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn typ(&self) -> &EntryType {
        &self.typ
    }
}

impl FromId<InstanceId> for Entry {
    fn from_id(problem: &Problem, id: InstanceId) -> Self {
        let name = problem.get(id).unwrap().name().into();
        let typ = EntryType::Instance(id);
        Self { name, typ }
    }
}

impl FromId<VariableId> for Entry {
    fn from_id(problem: &Problem, id: VariableId) -> Self {
        let name = problem.get(id).unwrap().name().into();
        let typ = EntryType::Variable(id);
        Self { name, typ }
    }
}

//------------------------- Entries -------------------------

#[derive(Clone, Debug)]
pub struct Entries(Vec<Entry>);

impl Entries {
    pub fn new(entries: Vec<Entry>) -> Self {
        Entries(entries)
    }

    fn entries(&self) -> &Vec<Entry> {
        let Entries(entries) = self;
        entries
    }

    pub fn add(&self, entry: Entry) -> Entries {
        let mut v = self.entries().clone();
        v.push(entry);
        Entries(v)
    }

    pub fn add_all(&self, entries: Entries) -> Entries {
        let Entries(others) = entries;
        let mut v = self.entries().clone();
        for entry in others {
            v.push(entry);
        }
        Entries(v)
    }

    pub fn get(&self, name: &str) -> Option<Entry> {
        for e in self.entries().iter().rev() {
            if e.name() == name {
                return Some(e.clone());
            }
        }
        None
    }
}
