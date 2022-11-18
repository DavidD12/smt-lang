use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ConstraintId(pub usize);

impl Id for ConstraintId {
    fn empty() -> Self {
        Self(0)
    }
}

//------------------------- Constraint -------------------------

#[derive(Clone)]
pub struct Constraint {
    id: ConstraintId,
    name: String,
    expr: Expr,
    position: Option<Position>,
}

impl Constraint {
    pub fn new<S: Into<String>>(name: S, expr: Expr, position: Option<Position>) -> Self {
        let id = ConstraintId::empty();
        let name = name.into();
        Self {
            id,
            name,
            expr,
            position,
        }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    //---------- Resolve ----------

    pub fn resolve_type_expr(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        let e = self.expr.resolve_type(entries)?;
        self.expr = e;
        Ok(())
    }

    pub fn resolve_expr(&self, problem: &Problem, entries: &Entries) -> Result<Constraint, Error> {
        let expr = self.expr.resolve(problem, entries)?;
        Ok(Constraint {
            id: self.id,
            name: self.name.clone(),
            expr,
            position: self.position.clone(),
        })
    }

    //---------- Parameter Size ----------

    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        self.expr.check_parameter_size(problem)
    }

    //---------- Typing ----------

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        self.expr.check_type(problem)?;
        check_type_bool(&self.expr, &self.expr.typ(problem))?;
        Ok(())
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Constraint {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<ConstraintId> for Constraint {
    fn id(&self) -> ConstraintId {
        self.id
    }

    fn set_id(&mut self, id: ConstraintId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Constraint {
    fn to_lang(&self, problem: &Problem) -> String {
        format!(
            "constraint {} = {}",
            self.name(),
            self.expr.to_lang(problem)
        )
    }
}
