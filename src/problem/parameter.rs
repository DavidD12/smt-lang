use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ParameterId(pub FunctionId, pub usize);

impl Id for ParameterId {
    fn empty() -> Self {
        Self(FunctionId::empty(), 0)
    }
}

//------------------------- Parameter -------------------------

#[derive(Clone)]
pub struct Parameter {
    id: ParameterId,
    name: String,
    typ: Type,
    position: Option<Position>,
}

impl Parameter {
    pub fn new<S: Into<String>>(name: S, typ: Type, position: Option<Position>) -> Self {
        let id = ParameterId::empty();
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

    pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
        if self.typ.is_bounded() {
            Ok(())
        } else {
            Err(Error::Bounded {
                name: self.to_lang(problem),
                position: self.position.clone(),
            })
        }
    }
}

//------------------------- Named -------------------------

impl Named<ParameterId> for Parameter {
    fn id(&self) -> ParameterId {
        self.id
    }

    fn set_id(&mut self, id: ParameterId) {
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

impl ToLang for Parameter {
    fn to_lang(&self, problem: &Problem) -> String {
        format!("{}: {}", self.name(), self.typ.to_lang(problem))
    }
}
