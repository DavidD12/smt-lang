use super::*;
use crate::parser::Position;

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
    typ: Type,
    position: Option<Position>,
}

impl Instance {
    pub fn new<S: Into<String>>(name: S, typ: Type, position: Option<Position>) -> Self {
        let id = InstanceId::empty();
        let name = name.into();
        Self {
            id,
            name,
            typ,
            position,
        }
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

//------------------------- WithType -------------------------

impl WithType for Instance {
    fn typ(&self) -> &Type {
        &self.typ
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = typ
    }

    fn resolve_type_children(&mut self, _: &TypeEntries) -> Result<(), Error> {
        Ok(())
    }

    fn check_interval_children(&self, _: &Problem) -> Result<(), Error> {
        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Instance {
    fn to_lang(&self, problem: &Problem) -> String {
        format!("inst {}: {}", self.name(), self.typ.to_lang(problem))
    }
}
