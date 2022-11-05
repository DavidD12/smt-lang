use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AttributeId(pub StructureId, pub usize);

impl Id for AttributeId {
    fn empty() -> Self {
        Self(StructureId::empty(), 0)
    }
}

//------------------------- Attribute -------------------------

#[derive(Clone)]
pub struct Attribute {
    id: AttributeId,
    name: String,
    typ: Type,
    expr: Option<Expr>,
    position: Option<Position>,
}

impl Attribute {
    pub fn new<S: Into<String>>(
        name: S,
        typ: Type,
        expr: Option<Expr>,
        position: Option<Position>,
    ) -> Self {
        let id = AttributeId::empty();
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

    pub fn resolve(&self, problem: &Problem, entries: &Entries) -> Result<Attribute, Error> {
        let expr = if let Some(e) = &self.expr {
            let resolved = e.resolve(problem, entries)?;
            Some(resolved)
        } else {
            None
        };
        Ok(Attribute {
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

impl Named<AttributeId> for Attribute {
    fn id(&self) -> AttributeId {
        self.id
    }

    fn set_id(&mut self, id: AttributeId) {
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

impl ToLang for Attribute {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("    {}: {}", self.name(), self.typ.to_lang(problem));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}
