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
}

//------------------------- Postion -------------------------

impl WithPosition for Variable {
    fn position(&self) -> &Option<Position> {
        &self.position
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
}

//------------------------- With Type -------------------------

impl WithType for Variable {
    fn typ(&self) -> &Type {
        &self.typ
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = typ;
    }

    fn resolve_type_children(&mut self, _: &TypeEntries) -> Result<(), Error> {
        Ok(())
    }

    fn check_interval_children(&self, _: &Problem) -> Result<(), Error> {
        Ok(())
    }
}

//------------------------- With Expr -------------------------

impl WithExpr for Variable {
    fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    fn clear_expr(&mut self) {
        self.expr = None;
    }

    fn new_expr(&self, expr: Option<Expr>) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            typ: self.typ.clone(),
            expr,
            position: self.position.clone(),
        }
    }

    fn entries(&self) -> Entries {
        Entries::new(vec![])
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Variable {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("let {}: {}", self.name(), self.typ.to_lang(problem));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}
