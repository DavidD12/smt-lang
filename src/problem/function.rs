use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct FunctionId(pub usize);

impl Id for FunctionId {
    fn empty() -> Self {
        Self(0)
    }
}

//------------------------- Function -------------------------

#[derive(Clone)]
pub struct Function {
    id: FunctionId,
    name: String,
    parameters: Vec<Parameter>,
    typ: Type,
    expr: Option<Expr>,
    position: Option<Position>,
}

impl Function {
    pub fn new<S: Into<String>>(
        name: S,
        typ: Type,
        expr: Option<Expr>,
        position: Option<Position>,
    ) -> Self {
        let id = FunctionId::empty();
        let name = name.into();
        Self {
            id,
            name,
            parameters: vec![],
            typ,
            expr,
            position,
        }
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn add_parameter(&mut self, mut parameter: Parameter) -> ParameterId {
        let id = ParameterId(self.id, self.parameters.len());
        parameter.set_id(id);
        self.parameters.push(parameter);
        id
    }

    pub fn typ(&self) -> Type {
        self.typ
    }

    pub fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    //---------- Resolve ----------

    pub fn resolve(&mut self, entries: &Entries) -> Result<(), Error> {
        if let Some(e) = &self.expr {
            let resolved = e.resolve(entries)?;
            self.expr = Some(resolved);
        }
        Ok(())
    }

    //---------- Interval ----------

    pub fn check_interval(&self, problem: &Problem) -> Result<(), Error> {
        match self.typ() {
            Type::Interval(min, max) => {
                if min > max {
                    Err(Error::Interval {
                        name: self.typ().to_lang(problem),
                        position: self.position.clone(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    //---------- Typing ----------

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        if let Some(e) = &self.expr {
            e.check_type(problem)?;
            check_same_type(self.typ(), e, e.typ(problem))?;
        }
        Ok(())
    }
}

//------------------------- Named -------------------------

impl Named<FunctionId> for Function {
    fn id(&self) -> FunctionId {
        self.id
    }

    fn set_id(&mut self, id: FunctionId) {
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

impl ToLang for Function {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("fun {}(", self.name());
        if !self.parameters.is_empty() {
            s.push_str(&format!(
                "{}",
                self.parameters.first().unwrap().to_lang(problem)
            ));
            for p in self.parameters[1..].iter() {
                s.push_str(&format!(", {}", p.to_lang(problem)));
            }
        }
        s.push_str(&format!("): {}", self.typ.to_lang(problem)));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}
