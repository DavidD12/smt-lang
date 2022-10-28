use super::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Int,
    Real,
}

impl ToLang for Type {
    fn to_lang(&self, problem: &Problem) -> String {
        match self {
            Type::Bool => "Bool".into(),
            Type::Int => "Int".into(),
            Type::Real => "Real".into(),
        }
    }
}
