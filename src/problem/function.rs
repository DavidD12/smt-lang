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
    return_type: Type,
    expr: Option<Expr>,
    position: Option<Position>,
}

impl Function {
    pub fn new<S: Into<String>>(
        name: S,
        return_type: Type,
        expr: Option<Expr>,
        position: Option<Position>,
    ) -> Self {
        let id = FunctionId::empty();
        let name = name.into();
        Self {
            id,
            name,
            parameters: vec![],
            return_type,
            expr,
            position,
        }
    }

    pub fn return_type(&self) -> Type {
        self.return_type.clone()
    }

    // pub fn typ(&self) -> Type {
    //     Type::Function(self.parameters_type(), Box::new(self.return_type()))
    // }

    pub fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    pub fn clear_expr(&mut self) {
        self.expr = None;
    }

    //---------- Parameter ----------

    pub fn add_parameter(&mut self, mut parameter: Parameter) -> ParameterId {
        let id = ParameterId(self.id, self.parameters.len());
        parameter.set_id(id);
        self.parameters.push(parameter);
        id
    }

    pub fn get_parameter(&self, id: ParameterId) -> Option<&Parameter> {
        let ParameterId(function_id, n) = id;
        if self.id != function_id {
            None
        } else {
            self.parameters.get(n)
        }
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn parameters_type(&self) -> Vec<Type> {
        self.parameters.iter().map(|p| p.typ().clone()).collect()
    }

    //---------- Duplicate ----------

    pub fn duplicate(&self) -> Result<(), Error> {
        for i in 0..self.parameters.len() - 1 {
            let x = &self.parameters[i];
            for j in i + 1..self.parameters.len() {
                let y = &self.parameters[j];
                if x.name() == y.name() {
                    return Err(Error::Duplicate {
                        name: x.name().to_string(),
                        first: x.position().clone(),
                        second: y.position().clone(),
                    });
                }
            }
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        self.return_type = self.return_type.resolve_type(entries)?;
        for p in self.parameters.iter_mut() {
            p.resolve_type(entries)?;
        }
        Ok(())
    }

    pub fn resolve_expr(&self, problem: &Problem, entries: &Entries) -> Result<Function, Error> {
        let expr = if let Some(e) = &self.expr {
            let mut entries = entries.clone();
            for p in self.parameters.iter() {
                let entry = Entry::new(p.name().to_string(), EntryType::Parameter(p.id()));
                entries = entries.add(entry);
            }
            let resolved = e.resolve(problem, &entries)?;
            Some(resolved)
        } else {
            None
        };
        Ok(Function {
            id: self.id,
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            return_type: self.return_type.clone(),
            expr,
            position: self.position.clone(),
        })
    }

    //---------- Interval ----------

    pub fn check_interval(&self, problem: &Problem) -> Result<(), Error> {
        self.return_type.check_interval(problem, &self.position)?;
        for p in self.parameters.iter() {
            p.check_interval(problem)?;
        }
        Ok(())
    }

    //---------- Parameter Size ----------

    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        if let Some(expr) = &self.expr {
            expr.check_parameter_size(problem)?;
        }
        Ok(())
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
        for p in self.parameters.iter() {
            p.check_bounded(problem)?;
        }
        Ok(())
    }

    //---------- Typing ----------

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        if let Some(e) = &self.expr {
            e.check_type(problem)?;
            check_compatible_type(&self.return_type, e, &e.typ(problem))?;
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
        let mut s = format!("let {}(", self.name());
        if !self.parameters.is_empty() {
            s.push_str(&format!(
                "{}",
                self.parameters.first().unwrap().to_lang(problem)
            ));
            for p in self.parameters[1..].iter() {
                s.push_str(&format!(", {}", p.to_lang(problem)));
            }
        }
        s.push_str(&format!("): {}", self.return_type.to_lang(problem)));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<ParameterId, Parameter> for Function {
    fn get(&self, id: ParameterId) -> Option<&Parameter> {
        self.get_parameter(id)
    }
}
