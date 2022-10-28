use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Bool,
    Int,
    Real,
    //
    Undefined,
}
//------------------------- To Lang -------------------------

impl ToLang for Type {
    fn to_lang(&self, _problem: &Problem) -> String {
        match self {
            Type::Bool => "Bool".into(),
            Type::Int => "Int".into(),
            Type::Real => "Real".into(),
            Type::Undefined => "Undefined".into(),
        }
    }
}
