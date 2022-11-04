use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ArgumentId(pub MethodId, pub usize);

impl Id for ArgumentId {
    fn empty() -> Self {
        Self(MethodId::empty(), 0)
    }
}

//------------------------- Argument -------------------------

#[derive(Clone)]
pub struct Argument {
    id: ArgumentId,
    name: String,
    typ: Type,
    position: Option<Position>,
}

impl Argument {
    pub fn new<S: Into<String>>(name: S, typ: Type, position: Option<Position>) -> Self {
        let id = ArgumentId::empty();
        let name = name.into();
        Self {
            id,
            name,
            typ,
            position,
        }
    }

    pub fn typ(&self) -> Type {
        self.typ.clone()
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        self.typ = self.typ.resolve_type(entries)?;
        Ok(())
    }

    //---------- Interval ----------

    pub fn check_interval(&self, problem: &Problem) -> Result<(), Error> {
        self.typ.check_interval(problem, &self.position)
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self, _: &Problem) -> Result<(), Error> {
        if self.typ.is_bounded() {
            Ok(())
        } else {
            Err(Error::Bounded {
                name: self.name.clone(),
                position: self.position.clone(),
            })
        }
    }
}

//------------------------- Named -------------------------

impl Named<ArgumentId> for Argument {
    fn id(&self) -> ArgumentId {
        self.id
    }

    fn set_id(&mut self, id: ArgumentId) {
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

impl ToLang for Argument {
    fn to_lang(&self, problem: &Problem) -> String {
        format!("{}: {}", self.name(), self.typ.to_lang(problem))
    }
}
