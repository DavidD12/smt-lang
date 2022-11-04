use super::*;

//------------------------- Entry Type -------------------------

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum EntryType {
    Variable(VariableId),
    Function(FunctionId),
    Parameter(ParameterId),
    Argument(ArgumentId),
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

    pub fn typ(&self) -> EntryType {
        self.typ
    }
}

impl FromId<VariableId> for Entry {
    fn from_id(problem: &Problem, id: VariableId) -> Self {
        let name = problem.get(id).unwrap().name().into();
        let typ = EntryType::Variable(id);
        Self { name, typ }
    }
}

impl FromId<FunctionId> for Entry {
    fn from_id(problem: &Problem, id: FunctionId) -> Self {
        let name = problem.get(id).unwrap().name().into();
        let typ = EntryType::Function(id);
        Self { name, typ }
    }
}

impl FromId<ParameterId> for Entry {
    fn from_id(problem: &Problem, id: ParameterId) -> Self {
        let name = problem.get(id).unwrap().name().into();
        let typ = EntryType::Parameter(id);
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

    pub fn get(&self, name: &str) -> Option<Entry> {
        for e in self.entries().iter().rev() {
            if e.name() == name {
                return Some(e.clone());
            }
        }
        None
    }
}
