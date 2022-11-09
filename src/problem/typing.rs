use std::vec;

use super::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Bool,
    Int,
    Real,
    Interval(isize, isize),
    Structure(StructureId),
    // Function(Vec<Type>, Box<Type>),
    //
    Unresolved(String, Option<Position>),
    Undefined,
}

impl Type {
    pub fn is_bounded(&self) -> bool {
        match self {
            Type::Bool => true,
            Type::Int => false,
            Type::Real => false,
            Type::Interval(_, _) => true,
            Type::Structure(_) => true,
            // Type::Function(_, _) => false,
            Type::Unresolved(_, _) => false,
            Type::Undefined => false,
        }
    }

    pub fn resolve_type(&self, entries: &TypeEntries) -> Result<Type, Error> {
        match self {
            Type::Bool => Ok(self.clone()),
            Type::Int => Ok(self.clone()),
            Type::Real => Ok(self.clone()),
            Type::Interval(_, _) => Ok(self.clone()),
            Type::Structure(_) => Ok(self.clone()),
            Type::Unresolved(name, position) => match entries.get(&name) {
                Some(entry) => match entry.typ() {
                    TypeEntryType::Structure(id) => Ok(Type::Structure(id)),
                    // _ => Err(Error::Resolve {
                    //     category: "type".to_string(),
                    //     name: name.clone(),
                    //     position: position.clone(),
                    // }),
                },
                None => Err(Error::Resolve {
                    category: "type".to_string(),
                    name: name.clone(),
                    position: position.clone(),
                }),
            },
            Type::Undefined => todo!(),
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

    pub fn is_structure(&self) -> bool {
        match self {
            Type::Structure(_) => true,
            _ => false,
        }
    }

    pub fn is_compatible_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Interval(_, _), Type::Interval(_, _)) => true,
            (Type::Interval(_, _), Type::Int) => true,
            (Type::Int, Type::Interval(_, _)) => true,
            (x, y) => x.is_subtype_of(y),
        }
    }

    pub fn is_subtype_of(&self, other: &Self) -> bool {
        match (self, other) {
            // TODO: Int :<: Real ??? and Interval :<: Real
            (Type::Interval(min1, max1), Type::Interval(min2, max2)) => {
                min1 >= min2 && max1 <= max2
            }
            (Type::Interval(_, _), Type::Int) => true,
            (x, y) => x == y,
        }
    }

    // pub fn conform_list(&self) -> Vec<Self> {
    //     match self {
    //         Type::Bool => vec![self.clone()],
    //         Type::Int => vec![self.clone()],
    //         Type::Real => vec![self.clone()],
    //         Type::Interval(_, _) => vec![self.clone()],
    //         Type::Function(_, _) => vec![self.clone()],
    //         Type::Undefined => vec![],
    //     }
    // }

    pub fn check_interval(
        &self,
        problem: &Problem,
        position: &Option<Position>,
    ) -> Result<(), Error> {
        match self {
            Type::Interval(min, max) => {
                if min > max {
                    Err(Error::Interval {
                        name: self.to_lang(problem),
                        position: position.clone(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    pub fn all(&self, problem: &Problem) -> Vec<Expr> {
        match self {
            Type::Structure(id) => problem
                .structure_instances(*id)
                .iter()
                .map(|i| Expr::Instance(*i, None))
                .collect(),
            Type::Bool => vec![Expr::BoolValue(false, None), Expr::BoolValue(true, None)],
            Type::Interval(min, max) => (*min..*max)
                .into_iter()
                .map(|i| Expr::IntValue(i, None))
                .collect(),
            _ => vec![],
        }
    }
}
//------------------------- To Lang -------------------------

impl ToLang for Type {
    fn to_lang(&self, problem: &Problem) -> String {
        match self {
            Type::Bool => "Bool".into(),
            Type::Int => "Int".into(),
            Type::Real => "Real".into(),
            Type::Interval(min, max) => format!("{}..{}", min, max),
            // Type::Function(param, ret) => {
            //     let mut s = "(".to_string();
            //     if let Some((first, others)) = param.split_first() {
            //         s.push_str(&first.to_lang(problem));
            //         for p in others {
            //             s.push_str(&format!(", {}", p.to_lang(problem)));
            //         }
            //     }
            //     s.push_str(&format!(") -> {}", ret.to_lang(problem)));
            //     s
            // }
            Type::Structure(id) => problem.get(*id).unwrap().name().to_string(),
            Type::Unresolved(name, _) => format!("{}?", name),
            Type::Undefined => "?".into(),
        }
    }
}
