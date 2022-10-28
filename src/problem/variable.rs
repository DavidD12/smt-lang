use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct VariableId(pub usize);

impl Id for VariableId {
    fn empty() -> Self {
        Self(0)
    }
}

//------------------------- Variable -------------------------

#[derive(Clone)]
pub struct Variable {
    id: VariableId,
    name: String,
    typ: Type,
    expr: Option<Expr>,
    position: Option<Position>,
}

impl Variable {
    pub fn new<S: Into<String>>(
        name: S,
        typ: Type,
        expr: Option<Expr>,
        position: Option<Position>,
    ) -> Self {
        let id = VariableId::empty();
        let name = name.into();
        Self {
            id,
            name,
            typ,
            expr,
            position,
        }
    }

    pub fn typ(&self) -> &Type {
        &self.typ
    }

    pub fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    //---------- Resolve ----------

    pub fn resolve(&mut self, entries: &Entries) -> Result<(), Error> {
        if let Some(e) = &self.expr {
            let resolved = e.resolve(entries)?;
            self.expr = Some(resolved);
        }
        Ok(())
    }
}

//------------------------- Named -------------------------

impl Named<VariableId> for Variable {
    fn id(&self) -> VariableId {
        self.id
    }

    fn set_id(&mut self, id: VariableId) {
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

impl ToLang for Variable {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("var {}: {}", self.name(), self.typ.to_lang(problem));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}
