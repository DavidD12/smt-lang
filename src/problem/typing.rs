use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Bool,
    Int,
    Real,
    Interval(isize, isize),
    //
    Undefined,
}

impl Type {
    pub fn is_bounded(&self) -> bool {
        match self {
            Type::Bool => true,
            Type::Int => false,
            Type::Real => false,
            Type::Interval(_, _) => true,
            Type::Undefined => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Type::Bool => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Type::Int => true,
            _ => false,
        }
    }

    pub fn is_real(&self) -> bool {
        match self {
            Type::Real => true,
            _ => false,
        }
    }

    pub fn is_interval(&self) -> bool {
        match self {
            Type::Bool => true,
            _ => false,
        }
    }

    pub fn is_undefined(&self) -> bool {
        match self {
            Type::Undefined => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Type::Int => true,
            Type::Interval(_, _) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Type::Int => true,
            Type::Real => true,
            Type::Interval(_, _) => true,
            _ => false,
        }
    }

    pub fn conform_to(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Bool, Type::Bool) => true,
            (Type::Int, Type::Int) => true,
            (Type::Int, Type::Interval(_, _)) => true,
            (Type::Real, Type::Real) => true,
            (Type::Interval(_, _), Type::Interval(_, _)) => true,
            (Type::Interval(_, _), Type::Int) => true,
            _ => false,
        }
    }

    pub fn conform_list(&self) -> Vec<Self> {
        match self {
            Type::Bool => vec![Type::Bool],
            Type::Int => vec![Type::Int],
            Type::Real => vec![Type::Real],
            Type::Interval(_, _) => vec![Type::Int],
            Type::Undefined => vec![],
        }
    }
}
//------------------------- To Lang -------------------------

impl ToLang for Type {
    fn to_lang(&self, _problem: &Problem) -> String {
        match self {
            Type::Bool => "Bool".into(),
            Type::Int => "Int".into(),
            Type::Real => "Real".into(),
            Type::Interval(min, max) => format!("{}..{}", min, max),
            Type::Undefined => "Undefined".into(),
        }
    }
}
