use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ParameterId<T: Id>(pub T, pub usize);

impl<T: Id> Id for ParameterId<T> {
    fn empty() -> Self {
        Self(T::empty(), 0)
    }
}

//------------------------- Parameter -------------------------

#[derive(Clone)]
pub struct Parameter<T: Id> {
    id: ParameterId<T>,
    name: String,
    typ: Type,
    position: Option<Position>,
}

impl<T: Id> Parameter<T> {
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

impl<T: Id> WithType for Parameter<T> {
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

impl<T: Id> WithPosition for Parameter<T> {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl<T: Id> Named<ParameterId<T>> for Parameter<T> {
    fn id(&self) -> ParameterId<T> {
        self.id
    }

    fn set_id(&mut self, id: ParameterId<T>) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- ToLang -------------------------

impl<T: Id> ToLang for Parameter<T> {
    fn to_lang(&self, problem: &Problem) -> String {
        format!("{}: {}", self.name(), self.typ.to_lang(problem))
    }
}
