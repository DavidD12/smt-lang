use super::*;
use crate::parser::Position;

//------------------------- Structure Ref -------------------------

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum StructureRef {
    Resolved(StructureId, Option<Position>),
    Unresolved(String, Option<Position>),
}

impl StructureRef {
    pub fn resolved(&self) -> StructureId {
        match self {
            StructureRef::Resolved(id, _) => *id,
            StructureRef::Unresolved(_, _) => panic!(),
        }
    }

    pub fn resolve(&self, problem: &Problem) -> Result<Self, Error> {
        match self {
            Self::Resolved(_, _) => Ok(self.clone()),
            Self::Unresolved(name, position) => {
                if let Some(structure) = problem.find_structure(name) {
                    Ok(Self::Resolved(structure.id(), position.clone()))
                } else {
                    Err(Error::Instance {
                        name: name.clone(),
                        position: position.clone(),
                    })
                }
            }
        }
    }
}

impl ToLang for StructureRef {
    fn to_lang(&self, problem: &Problem) -> String {
        match self {
            StructureRef::Resolved(id, _) => problem.get(*id).unwrap().name().to_string(),
            StructureRef::Unresolved(name, _) => format!("{}?", name),
        }
    }
}

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct InstanceId(pub usize);

impl Id for InstanceId {
    fn empty() -> Self {
        Self(0)
    }
}

//------------------------- Instance -------------------------

#[derive(Clone)]
pub struct Instance {
    id: InstanceId,
    name: String,
    structure: StructureRef,
    position: Option<Position>,
}

impl Instance {
    pub fn new<S: Into<String>>(
        name: S,
        structure: StructureRef,
        position: Option<Position>,
    ) -> Self {
        let id = InstanceId::empty();
        let name = name.into();
        Self {
            id,
            name,
            structure,
            position,
        }
    }

    pub fn structure(&self) -> &StructureRef {
        &self.structure
    }

    pub fn typ(&self) -> Type {
        Type::Structure(self.structure.resolved())
    }

    //---------- Resolve ----------

    pub fn resolve_instance(&self, problem: &Problem) -> Result<Instance, Error> {
        let structure = self.structure.resolve(problem)?;
        Ok(Instance {
            id: self.id(),
            name: self.name.clone(),
            structure,
            position: self.position.clone(),
        })
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Instance {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<InstanceId> for Instance {
    fn id(&self) -> InstanceId {
        self.id
    }

    fn set_id(&mut self, id: InstanceId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Instance {
    fn to_lang(&self, problem: &Problem) -> String {
        format!("inst {}: {}", self.name(), self.structure.to_lang(problem))
    }
}
