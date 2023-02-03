use super::*;
use fraction::Fraction;

#[derive(Clone)]
pub enum Bound {
    Int(isize),
    Real(Fraction),
}

impl std::fmt::Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bound::Int(v) => write!(f, "{}", v),
            Bound::Real(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone)]
pub enum Search {
    Solve,
    Optimize(Box<Expr>, Bound, bool),
}

impl Search {
    pub fn resolve_type_expr(&self, entries: &TypeEntries) -> Result<Self, Error> {
        match self {
            Search::Solve => Ok(self.clone()),
            Search::Optimize(e, bound, minimize) => {
                let e = e.resolve_type(entries)?;
                Ok(Search::Optimize(Box::new(e), bound.clone(), *minimize))
            }
        }
    }

    pub fn resolve_expr(&self, problem: &Problem, entries: &Entries) -> Result<Self, Error> {
        match self {
            Search::Solve => Ok(self.clone()),
            Search::Optimize(e, bound, minimize) => {
                let e = e.resolve(problem, entries)?;
                Ok(Search::Optimize(Box::new(e), bound.clone(), *minimize))
            }
        }
    }

    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Search::Solve => Ok(()),
            Search::Optimize(e, _, _) => e.check_parameter_size(problem),
        }
    }

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Search::Solve => Ok(()),
            Search::Optimize(e, bound, _) => {
                e.check_type(problem)?;
                let et = e.typ(problem);
                match bound {
                    Bound::Int(_) => check_type_integer(e, &et),
                    Bound::Real(_) => check_type_real(e, &et),
                }
            }
        }
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Search {
    fn to_lang(&self, problem: &Problem) -> String {
        match self {
            Search::Solve => "solve\n".to_string(),
            Search::Optimize(e, bound, minimize) => {
                if *minimize {
                    format!("minimize {} until {}", e.to_lang(problem), bound)
                } else {
                    format!("maximize {} until {}", e.to_lang(problem), bound)
                }
            }
        }
    }
}
