use super::*;

#[derive(Clone)]
pub struct Problem {
    structures: Vec<Structure>,
    variables: Vec<Variable>,
    functions: Vec<Function>,
    constraints: Vec<Constraint>,
}

impl Problem {
    pub fn new() -> Self {
        Self {
            structures: vec![],
            variables: vec![],
            functions: vec![],
            constraints: vec![],
        }
    }

    //---------- Structure ----------

    pub fn add_structure(&mut self, mut structure: Structure) -> StructureId {
        let id = StructureId(self.structures.len());
        structure.set_id(id);
        self.structures.push(structure);
        id
    }

    pub fn get_structure(&self, id: StructureId) -> Option<&Structure> {
        let StructureId(n) = id;
        self.structures.get(n)
    }

    pub fn structures(&self) -> &Vec<Structure> {
        &self.structures
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

    //---------- Constraint ----------

    pub fn add_constraint(&mut self, mut constraint: Constraint) -> ConstraintId {
        let id = ConstraintId(self.constraints.len());
        constraint.set_id(id);
        self.constraints.push(constraint);
        id
    }

    pub fn get_constraint(&self, id: ConstraintId) -> Option<&Constraint> {
        let ConstraintId(n) = id;
        self.constraints.get(n)
    }

    pub fn constraints(&self) -> &Vec<Constraint> {
        &self.constraints
    }

    //---------- Entry ----------

    pub fn type_entries(&self) -> TypeEntries {
        let mut v = vec![];
        v.extend(
            self.structures
                .iter()
                .map(|x| TypeEntry::from_id(self, x.id())),
        );
        TypeEntries::new(v)
    }

    pub fn entries(&self) -> Entries {
        let mut v = vec![];
        v.extend(self.variables.iter().map(|x| Entry::from_id(self, x.id())));
        v.extend(self.functions.iter().map(|x| Entry::from_id(self, x.id())));
        Entries::new(v)
    }

    //---------- Duplicate ----------

    pub fn naming(&self) -> Vec<Naming> {
        let mut v = vec![];
        v.extend(self.structures.iter().map(|x| x.naming()));
        v.extend(self.variables.iter().map(|x| x.naming()));
        v.extend(self.functions.iter().map(|x| x.naming()));
        v.extend(self.constraints.iter().map(|x| x.naming()));
        v
    }

    pub fn duplicate(&self) -> Result<(), Error> {
        check_duplicate(self.naming())?;
        for x in self.structures.iter() {
            x.duplicate()?;
        }
        for x in self.functions.iter() {
            x.duplicate()?;
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self) -> Result<(), Error> {
        let entries = self.type_entries();
        for x in self.structures.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.variables.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.functions.iter_mut() {
            x.resolve_type(&entries)?;
        }
        Ok(())
    }

    pub fn resolve(&mut self) -> Result<(), Error> {
        let entries = self.entries();
        for x in self.structures.iter_mut() {
            x.resolve(&entries)?;
        }
        for x in self.variables.iter_mut() {
            x.resolve(&entries)?;
        }
        for x in self.functions.iter_mut() {
            x.resolve(&entries)?;
        }
        for x in self.constraints.iter_mut() {
            x.resolve(&entries)?;
        }
        Ok(())
    }

    //---------- Interval ----------

    pub fn check_interval(&self) -> Result<(), Error> {
        for x in self.variables.iter() {
            x.check_interval(self)?;
        }
        for x in self.functions.iter() {
            x.check_interval(self)?;
        }
        Ok(())
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self) -> Result<(), Error> {
        for x in self.functions.iter() {
            x.check_bounded(self)?;
        }
        Ok(())
    }

    //---------- Typing ----------

    pub fn check_type(&self) -> Result<(), Error> {
        for x in self.variables.iter() {
            x.check_type(self)?;
        }
        for x in self.functions.iter() {
            x.check_type(self)?;
        }
        for x in self.constraints.iter() {
            x.check_type(self)?;
        }
        Ok(())
    }

    //---------- To Entry ----------

    pub fn to_entry(&self) -> d_stuff::Entry {
        d_stuff::Entry::new(
            d_stuff::Status::Info,
            d_stuff::Text::new(
                "Problem",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![d_stuff::Message::new(
                None,
                d_stuff::Text::new(
                    format!("{}", self),
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                ),
            )],
        )
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.structures.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        for x in self.variables.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        for x in self.functions.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        for x in self.constraints.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        Ok(())
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<StructureId, Structure> for Problem {
    fn get(&self, id: StructureId) -> Option<&Structure> {
        self.get_structure(id)
    }
}

impl GetFromId<VariableId, Variable> for Problem {
    fn get(&self, id: VariableId) -> Option<&Variable> {
        self.get_variable(id)
    }
}

impl GetFromId<FunctionId, Function> for Problem {
    fn get(&self, id: FunctionId) -> Option<&Function> {
        self.get_function(id)
    }
}
impl GetFromId<ParameterId, Parameter> for Problem {
    fn get(&self, id: ParameterId) -> Option<&Parameter> {
        let ParameterId(function_id, _) = id;
        if let Some(function) = self.get(function_id) {
            function.get(id)
        } else {
            None
        }
    }
}

impl GetFromId<ConstraintId, Constraint> for Problem {
    fn get(&self, id: ConstraintId) -> Option<&Constraint> {
        self.get_constraint(id)
    }
}
