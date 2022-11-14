use super::*;
use crate::parser::Position;

//------------------------- Parameter -------------------------

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Parameter {
    name: String,
    typ: Type,
    position: Option<Position>,
}

impl Parameter {
    pub fn new<S: Into<String>>(name: S, typ: Type, position: Option<Position>) -> Self {
        let name = name.into();
        Self {
            name,
            typ,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
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

//------------------------- With Type -------------------------

impl WithType for Parameter {
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

//------------------------- Postion -------------------------

impl WithPosition for Parameter {
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
