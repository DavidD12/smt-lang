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
    parameters: Vec<Parameter<FunctionId>>,
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

    //---------- Parameter ----------

    pub fn add_parameter(
        &mut self,
        mut parameter: Parameter<FunctionId>,
    ) -> ParameterId<FunctionId> {
        let id = ParameterId(self.id, self.parameters.len());
        parameter.set_id(id);
        self.parameters.push(parameter);
        id
    }

    pub fn get_parameter(&self, id: ParameterId<FunctionId>) -> Option<&Parameter<FunctionId>> {
        let ParameterId(function_id, n) = id;
        if self.id != function_id {
            None
        } else {
            self.parameters.get(n)
        }
    }

    pub fn parameters(&self) -> &Vec<Parameter<FunctionId>> {
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

    //---------- Bounded ----------

    pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
        for p in self.parameters.iter() {
            p.check_bounded(problem)?;
        }
        Ok(())
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Function {
    fn position(&self) -> &Option<Position> {
        &self.position
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
}

//------------------------- With Type -------------------------

impl WithType for Function {
    fn typ(&self) -> &Type {
        &self.typ
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = typ;
    }

    fn resolve_type_children(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        for p in self.parameters.iter_mut() {
            p.resolve_type(entries)?;
        }
        Ok(())
    }

    fn check_interval_children(&self, problem: &Problem) -> Result<(), Error> {
        for p in self.parameters.iter() {
            p.check_interval(problem)?;
        }
        Ok(())
    }
}

//------------------------- With Expr -------------------------

impl WithExpr for Function {
    fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    fn clear_expr(&mut self) {
        self.expr = None;
    }

    fn new_expr(&self, expr: Option<Expr>) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            typ: self.typ.clone(),
            expr,
            position: self.position.clone(),
        }
    }

    fn entries(&self) -> Entries {
        let mut v = Vec::new();
        for p in self.parameters.iter() {
            v.push(Entry::new(
                p.name().to_string(),
                EntryType::FunParam(p.id()),
            ));
        }
        Entries::new(v)
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
        s.push_str(&format!("): {}", self.typ.to_lang(problem)));
        if let Some(e) = &self.expr {
            s.push_str(&format!(" = {}", e.to_lang(problem)));
        }
        s
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<ParameterId<FunctionId>, Parameter<FunctionId>> for Function {
    fn get(&self, id: ParameterId<FunctionId>) -> Option<&Parameter<FunctionId>> {
        self.get_parameter(id)
    }
}
