use super::*;

#[derive(Clone)]
pub struct Problem {
    variables: Vec<Variable>,
    functions: Vec<Function>,
}

impl Problem {
    pub fn new() -> Self {
        Self {
            variables: vec![],
            functions: vec![],
        }
    }

    //---------- Variable ----------

    pub fn add_variable(&mut self, mut variable: Variable) -> VariableId {
        let id = VariableId(self.variables.len());
        variable.set_id(id);
        self.variables.push(variable);
        id
    }

    pub fn get_variable(&self, id: VariableId) -> Option<&Variable> {
        let VariableId(n) = id;
        self.variables.get(n)
    }

    pub fn variables(&self) -> &Vec<Variable> {
        &self.variables
    }

    //---------- Function ----------

    pub fn add_function(&mut self, mut function: Function) -> FunctionId {
        let id = FunctionId(self.functions.len());
        function.set_id(id);
        self.functions.push(function);
        id
    }

    pub fn get_function(&self, id: FunctionId) -> Option<&Function> {
        let FunctionId(n) = id;
        self.functions.get(n)
    }

    pub fn functions(&self) -> &Vec<Function> {
        &self.functions
    }

    //---------- Entry ----------

    pub fn entries(&self) -> Entries {
        let mut v = vec![];
        v.extend(self.variables.iter().map(|x| Entry::from_id(self, x.id())));
        v.extend(self.functions.iter().map(|x| Entry::from_id(self, x.id())));
        Entries::new(v)
    }

    //---------- Duplicate ----------

    pub fn naming(&self) -> Vec<Naming> {
        let mut v = vec![];
        v.extend(self.variables.iter().map(|x| x.naming()));
        v
    }

    pub fn duplicate(&self) -> Result<(), Error> {
        check_duplicate(self.naming())
    }

    //---------- Resolve ----------

    pub fn resolve(&mut self) -> Result<(), Error> {
        let entries = self.entries();
        for x in self.variables.iter_mut() {
            x.resolve(&entries)?;
        }
        for x in self.functions.iter_mut() {
            x.resolve(&entries)?;
        }
        Ok(())
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.variables.iter() {
            write!(f, "{}\n", v.to_lang(self))?;
        }
        Ok(())
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<VariableId, Variable> for Problem {
    fn get(&self, i: VariableId) -> Option<&Variable> {
        self.get_variable(i)
    }
}

impl GetFromId<FunctionId, Function> for Problem {
    fn get(&self, i: FunctionId) -> Option<&Function> {
        self.get_function(i)
    }
}