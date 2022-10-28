pub mod problem;
pub use problem::*;

pub mod typing;
pub use typing::*;

pub mod variable;
pub use variable::*;

pub mod function;
pub use function::*;

pub mod expression;
pub use expression::*;

pub mod entry;
pub use entry::*;

//------------------------- Id -------------------------

pub trait Id: Clone + Copy + PartialEq + Eq + core::hash::Hash + std::fmt::Debug {
    fn empty() -> Self;
}

pub trait GetFromId<I: Id, T> {
    fn get(&self, i: I) -> Option<&T>;
}

pub trait FromId<I: Id> {
    fn from_id(problem: &Problem, id: I) -> Self;
}

//------------------------- Named -------------------------

use crate::error::Error;
use crate::parser::Position;

pub trait Named<I: Id> {
    fn id(&self) -> I;
    fn set_id(&mut self, id: I);
    fn name(&self) -> &str;
    fn position(&self) -> &Option<Position>;
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
