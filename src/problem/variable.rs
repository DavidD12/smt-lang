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

    pub fn typ(&self) -> Type {
        self.typ.clone()
    }

    pub fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    pub fn clear_expr(&mut self) {
        self.expr = None;
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        self.typ = self.typ.resolve_type(entries)?;
        Ok(())
    }

    pub fn resolve(&self, problem: &Problem, entries: &Entries) -> Result<Variable, Error> {
        let expr = if let Some(e) = &self.expr {
            let resolved = e.resolve(problem, entries)?;
            Some(resolved)
        } else {
            None
        };
        Ok(Variable {
            id: self.id,
            name: self.name.clone(),
            typ: self.typ.clone(),
            expr,
            position: self.position.clone(),
        })
    }

    //---------- Interval ----------

    pub fn check_interval(&self, problem: &Problem) -> Result<(), Error> {
        self.typ.check_interval(problem, &self.position)
    }

    //---------- Typing ----------

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        if let Some(e) = &self.expr {
            e.check_type(problem)?;
            check_compatible_type(&self.typ, e, &e.typ(problem))?;
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
        let mut s = format!("let {}: {}", self.name(), self.typ.to_lang(problem));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}
