pub mod problem;
pub use problem::*;

pub mod typing;
pub use typing::*;

pub mod variable;
pub use variable::*;

pub mod function;
pub use function::*;
pub mod parameter;
pub use parameter::*;

pub mod attribute;
pub use attribute::*;
pub mod method;
pub use method::*;

pub mod structure;
pub use structure::*;
pub mod instance;
pub use instance::*;

pub mod class;
pub use class::*;

pub mod constraint;
pub use constraint::*;

pub mod expression;
pub use expression::*;

pub mod entry;
pub use entry::*;
pub mod type_entry;
pub use type_entry::*;

pub mod search;
pub use search::*;

//------------------------- With Type -------------------------

pub trait WithType: WithPosition {
    fn typ(&self) -> &Type;
    fn set_type(&mut self, typ: Type);

    //---------- Resolve ----------

    fn resolve_type_children(&mut self, entries: &TypeEntries) -> Result<(), Error>;

    fn resolve_type(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        self.set_type(self.typ().resolve_type(entries)?);
        self.resolve_type_children(entries)
    }

    //---------- Interval ----------

    fn check_interval_children(&self, problem: &Problem) -> Result<(), Error>;

    fn check_interval(&self, problem: &Problem) -> Result<(), Error> {
        self.typ().check_interval(problem, self.position())?;
        self.check_interval_children(problem)
    }
}

//------------------------- With Expr -------------------------

pub trait WithPosition {
    fn position(&self) -> &Option<Position>;
}

pub trait WithExpr: Clone + Sized + WithPosition + WithType {
    fn expr(&self) -> &Option<Expr>;
    fn set_expr(&mut self, expr: Option<Expr>);

    //---------- Resolve ----------

    fn entries(&self) -> Entries;

    fn resolve_type_expr(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        if let Some(e) = &self.expr() {
            let resolved = e.resolve_type(&entries)?;
            self.set_expr(Some(resolved));
        };
        Ok(())
    }

    fn resolve_expr(&self, problem: &Problem, entries: &Entries) -> Result<Self, Error> {
        let mut x = self.clone();
        if let Some(e) = &self.expr() {
            let entries = entries.add_all(self.entries());
            let resolved = e.resolve(problem, &entries)?;
            x.set_expr(Some(resolved));
        };
        Ok(x)
    }

    //---------- Parameter Size ----------

    fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        if let Some(expr) = &self.expr() {
            expr.check_parameter_size(problem)?;
        }
        Ok(())
    }

    //---------- Typing ----------

    fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        if let Some(e) = &self.expr() {
            e.check_type(problem)?;
            check_compatible_type(problem, self.typ(), e, &e.typ(problem))?;
        }
        Ok(())
    }
}

//------------------------- Id -------------------------

pub trait Id: Clone + Copy + PartialEq + Eq + core::hash::Hash + std::fmt::Debug {
    fn empty() -> Self;
}

pub trait GetFromId<I: Id, T> {
    fn get(&self, id: I) -> Option<&T>;
}

pub trait FromId<I: Id> {
    fn from_id(problem: &Problem, id: I) -> Self;
}

//------------------------- Named -------------------------

use crate::error::Error;
use crate::parser::Position;

pub trait Named<I: Id>: WithPosition {
    fn id(&self) -> I;
    fn set_id(&mut self, id: I);
    //
    fn name(&self) -> &str;
    fn naming(&self) -> Naming {
        (self.name().into(), self.position().clone())
    }
}

pub type Naming = (String, Option<Position>);

pub fn check_duplicate(names: Vec<Naming>) -> Result<(), Error> {
    for (i, (n1, p1)) in names.iter().enumerate() {
        for (n2, p2) in names.iter().skip(i + 1) {
            if n1 == n2 {
                return Err(Error::Duplicate {
                    name: n1.clone(),
                    first: p1.clone(),
                    second: p2.clone(),
                });
            }
        }
    }
    Ok(())
}

//------------------------- ToLang -------------------------

pub trait ToLang {
    fn to_lang(&self, problem: &Problem) -> String;
}

//------------------------- ToEntry -------------------------

pub trait ToEntry {
    fn to_entry(&self, problem: &Problem) -> d_stuff::Entry;
}
