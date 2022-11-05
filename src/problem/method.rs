use super::*;
use crate::parser::Position;

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct MethodId(pub StructureId, pub usize);

impl Id for MethodId {
    fn empty() -> Self {
        Self(StructureId::empty(), 0)
    }
}

//------------------------- Method -------------------------

#[derive(Clone)]
pub struct Method {
    id: MethodId,
    name: String,
    arguments: Vec<Argument>,
    return_type: Type,
    expr: Option<Expr>,
    position: Option<Position>,
}

impl Method {
    pub fn new<S: Into<String>>(
        name: S,
        return_type: Type,
        expr: Option<Expr>,
        position: Option<Position>,
    ) -> Self {
        let id = MethodId::empty();
        let name = name.into();
        Self {
            id,
            name,
            arguments: vec![],
            return_type,
            expr,
            position,
        }
    }

    pub fn return_type(&self) -> Type {
        self.return_type.clone()
    }

    pub fn expr(&self) -> &Option<Expr> {
        &self.expr
    }

    pub fn clear_expr(&mut self) {
        self.expr = None;
    }

    //---------- Argument ----------

    pub fn add_argument(&mut self, mut argument: Argument) -> ArgumentId {
        let id = ArgumentId(self.id, self.arguments.len());
        argument.set_id(id);
        self.arguments.push(argument);
        id
    }

    pub fn get_argument(&self, id: ArgumentId) -> Option<&Argument> {
        let ArgumentId(method_id, n) = id;
        if self.id != method_id {
            None
        } else {
            self.arguments.get(n)
        }
    }

    pub fn arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn arguments_type(&self) -> Vec<Type> {
        self.arguments.iter().map(|p| p.typ().clone()).collect()
    }

    //---------- Duplicate ----------

    pub fn duplicate(&self) -> Result<(), Error> {
        for i in 0..self.arguments.len() - 1 {
            let x = &self.arguments[i];
            for j in i + 1..self.arguments.len() {
                let y = &self.arguments[j];
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
        for p in self.arguments.iter_mut() {
            p.resolve_type(entries)?;
        }
        Ok(())
    }

    pub fn resolve(&self, problem: &Problem, entries: &Entries) -> Result<Method, Error> {
        let expr = if let Some(e) = &self.expr {
            let mut entries = entries.clone();
            for p in self.arguments.iter() {
                let entry = Entry::new(p.name().to_string(), EntryType::Argument(p.id()));
                entries = entries.add(entry);
            }
            let resolved = e.resolve(problem, &entries)?;
            Some(resolved)
        } else {
            None
        };
        Ok(Method {
            id: self.id,
            name: self.name.clone(),
            arguments: self.arguments.clone(),
            return_type: self.return_type.clone(),
            expr,
            position: self.position.clone(),
        })
    }

    //---------- Interval ----------

    pub fn check_interval(&self, problem: &Problem) -> Result<(), Error> {
        self.return_type.check_interval(problem, &self.position)?;
        for p in self.arguments.iter() {
            p.check_interval(problem)?;
        }
        Ok(())
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
        for p in self.arguments.iter() {
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

impl Named<MethodId> for Method {
    fn id(&self) -> MethodId {
        self.id
    }

    fn set_id(&mut self, id: MethodId) {
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

impl ToLang for Method {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("    {}(", self.name());
        if !self.arguments.is_empty() {
            s.push_str(&format!(
                "{}",
                self.arguments.first().unwrap().to_lang(problem)
            ));
            for p in self.arguments[1..].iter() {
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
