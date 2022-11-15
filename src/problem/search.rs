use super::*;

#[derive(Clone)]
pub enum Search {
    Solve,
    Optimize(Box<Expr>, bool),
}

impl Search {
    pub fn resolve_expr(&self, problem: &Problem, entries: &Entries) -> Result<Self, Error> {
        match self {
            Search::Solve => Ok(self.clone()),
            Search::Optimize(e, minimize) => {
                let e = e.resolve(problem, entries)?;
                Ok(Search::Optimize(Box::new(e), *minimize))
            }
        }
    }

    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Search::Solve => Ok(()),
            Search::Optimize(e, _) => e.check_parameter_size(problem),
        }
    }

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Search::Solve => Ok(()),
            Search::Optimize(e, _) => {
                e.check_type(problem)?;
                let et = e.typ(problem);
                check_type_integer(e, &et)
            }
        }
    }

    pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Search::Solve => Ok(()),
            Search::Optimize(e, _) => {
                if e.typ(problem).is_bounded() {
                    Ok(())
                } else {
                    Err(Error::Bounded {
                        name: self.to_lang(problem),
                        position: e.position().clone(),
                    })
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
            Search::Optimize(e, minimize) => {
                if *minimize {
                    format!("minimize {}", e.to_lang(problem))
                } else {
                    format!("maximize {}", e.to_lang(problem))
                }
            }
        }
    }
}
