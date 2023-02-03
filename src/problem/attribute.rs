use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct AttributeId<T: Id>(pub T, pub usize);

impl<T: Id> Id for AttributeId<T> {
    fn empty() -> Self {
        Self(T::empty(), 0)
    }
}

//------------------------- Attribute -------------------------

#[derive(Clone)]
pub struct Attribute<T: Id> {
    id: AttributeId<T>,
    name: String,
    typ: Type,
    expr: Option<Expr>,
    position: Option<Position>,
}

impl<T: Id> Attribute<T> {
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

    //---------- Resolve ----------

    pub fn resolve_type_expr(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        if let Some(expr) = &self.expr {
            let e = expr.resolve_type(entries)?;
            self.expr = Some(e);
        }
        Ok(())
    }
}

//------------------------- Postion -------------------------

impl<T: Id> WithPosition for Attribute<T> {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}
//------------------------- Named -------------------------

impl<T: Id> Named<AttributeId<T>> for Attribute<T> {
    fn id(&self) -> AttributeId<T> {
        self.id
    }

    fn set_id(&mut self, id: AttributeId<T>) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- With Type -------------------------

impl<T: Id> WithType for Attribute<T> {
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

impl WithExpr for Attribute<StructureId> {
    fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    fn set_expr(&mut self, expr: Option<Expr>) {
        self.expr = expr;
    }

    fn entries(&self) -> Entries {
        let AttributeId(structure_id, _) = self.id();
        Entries::new(vec![Entry::new(
            "self".to_string(),
            EntryType::StrucSelf(structure_id),
        )])
    }
}

impl WithExpr for Attribute<ClassId> {
    fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    fn set_expr(&mut self, expr: Option<Expr>) {
        self.expr = expr;
    }

    fn entries(&self) -> Entries {
        let AttributeId(class_id, _) = self.id();
        Entries::new(vec![Entry::new(
            "self".to_string(),
            EntryType::ClassSelf(class_id),
        )])
    }
}

//------------------------- ToLang -------------------------

impl<T: Id> ToLang for Attribute<T> {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("    {}: {}", self.name(), self.typ.to_lang(problem));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}
