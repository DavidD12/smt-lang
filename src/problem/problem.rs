use std::collections::VecDeque;

use super::*;

#[derive(Clone)]
pub struct Problem {
    structures: Vec<Structure>,
    instances: Vec<Instance>,
    classes: Vec<Class>,
    variables: Vec<Variable>,
    functions: Vec<Function>,
    constraints: Vec<Constraint>,
    search: Search,
}

impl Problem {
    pub fn new() -> Self {
        Self {
            structures: vec![],
            classes: vec![],
            instances: vec![],
            variables: vec![],
            functions: vec![],
            constraints: vec![],
            search: Search::Solve,
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

    pub fn find_structure(&self, name: &str) -> Option<&Structure> {
        self.structures.iter().find(|x| x.name() == name)
    }

    //---------- Instance ----------

    pub fn add_instance(&mut self, mut instance: Instance) -> InstanceId {
        let id = InstanceId(self.instances.len());
        instance.set_id(id);
        self.instances.push(instance);
        id
    }

    pub fn get_instance(&self, id: InstanceId) -> Option<&Instance> {
        let InstanceId(n) = id;
        self.instances.get(n)
    }

    pub fn instances(&self) -> &Vec<Instance> {
        &self.instances
    }

    pub fn find_instance(&self, name: &str) -> Option<&Instance> {
        self.instances.iter().find(|x| x.name() == name)
    }

    //---------- Class ----------

    pub fn add_class(&mut self, mut class: Class) -> ClassId {
        let id = ClassId(self.classes.len());
        class.set_id(id);
        self.classes.push(class);
        id
    }

    pub fn get_class(&self, id: ClassId) -> Option<&Class> {
        let ClassId(n) = id;
        self.classes.get(n)
    }

    pub fn classes(&self) -> &Vec<Class> {
        &self.classes
    }

    pub fn find_class(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|x| x.name() == name)
    }

    pub fn classes_ordered(&self) -> Vec<ClassId> {
        let mut all = self.classes.iter().map(|c| c.id()).collect::<VecDeque<_>>();
        let mut v = Vec::new();
        while !all.is_empty() {
            let id = all.pop_front().unwrap();
            let c = self.get_class(id).unwrap();
            if let Some(super_id) = c.super_class() {
                if v.contains(&super_id) {
                    v.push(id);
                } else {
                    all.push_back(id);
                }
            } else {
                v.push(id);
            }
        }
        v
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

    pub fn find_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.iter().find(|x| x.name() == name)
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

    pub fn find_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|x| x.name() == name)
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

    //---------- Search ----------

    pub fn search(&self) -> &Search {
        &self.search
    }

    pub fn set_search(&mut self, search: Search) {
        self.search = search
    }

    //---------- Entry ----------

    pub fn type_entries(&self) -> TypeEntries {
        let mut v = vec![];
        v.extend(
            self.structures
                .iter()
                .map(|x| TypeEntry::from_id(self, x.id())),
        );
        v.extend(
            self.classes
                .iter()
                .map(|x| TypeEntry::from_id(self, x.id())),
        );
        TypeEntries::new(v)
    }

    pub fn entries(&self) -> Entries {
        let mut v = vec![];
        v.extend(self.instances.iter().map(|x| Entry::from_id(self, x.id())));
        v.extend(self.variables.iter().map(|x| Entry::from_id(self, x.id())));
        Entries::new(v)
    }

    //---------- Duplicate ----------

    pub fn naming(&self) -> Vec<Naming> {
        let mut v = vec![];
        v.extend(self.structures.iter().map(|x| x.naming()));
        v.extend(self.classes.iter().map(|x| x.naming()));
        v.extend(self.instances.iter().map(|x| x.naming()));
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
        for x in self.classes.iter() {
            x.duplicate(self)?;
        }
        for x in self.functions.iter() {
            x.duplicate()?;
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self) -> Result<(), Error> {
        let entries = self.type_entries();
        // Types
        for x in self.structures.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.classes.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.instances.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.variables.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.functions.iter_mut() {
            x.resolve_type(&entries)?;
        }
        // Expr
        for x in self.structures.iter_mut() {
            x.resolve_type_expr(&entries)?;
        }
        for x in self.classes.iter_mut() {
            x.resolve_type_expr(&entries)?;
        }
        for x in self.variables.iter_mut() {
            x.resolve_type_expr(&entries)?;
        }
        for x in self.functions.iter_mut() {
            x.resolve_type_expr(&entries)?;
        }
        for x in self.constraints.iter_mut() {
            x.resolve_type_expr(&entries)?;
        }
        // Optimize
        let s = self.search.resolve_type_expr(&entries)?;
        self.search = s;
        //
        Ok(())
    }

    pub fn resolve_expr(&mut self) -> Result<(), Error> {
        let entries = self.entries();
        // Structure
        let mut structures = Vec::new();
        for x in self.structures.iter() {
            let s = x.resolve_expr(self, &entries)?;
            structures.push(s);
        }
        self.structures = structures;
        // Class
        let mut classes = Vec::new();
        for x in self.classes.iter() {
            let c = x.resolve_expr(self, &entries)?;
            classes.push(c);
        }
        self.classes = classes;
        // Variables
        let mut variables = Vec::new();
        for x in self.variables.iter() {
            let v = x.resolve_expr(self, &entries)?;
            variables.push(v);
        }
        self.variables = variables;
        // Function
        let mut functions = Vec::new();
        for x in self.functions.iter() {
            let f = x.resolve_expr(self, &entries)?;
            functions.push(f);
        }
        self.functions = functions;
        // Constraint
        let mut constraints = Vec::new();
        for x in self.constraints.iter() {
            let c = x.resolve_expr(self, &entries)?;
            constraints.push(c);
        }
        self.constraints = constraints;
        // Search
        self.search = self.search.resolve_expr(self, &entries)?;
        //
        Ok(())
    }

    //---------- Interval ----------

    pub fn check_interval(&self) -> Result<(), Error> {
        for x in self.structures.iter() {
            x.check_interval(self)?;
        }
        for x in self.classes.iter() {
            x.check_interval(self)?;
        }
        for x in self.variables.iter() {
            x.check_interval(self)?;
        }
        for x in self.functions.iter() {
            x.check_interval(self)?;
        }
        Ok(())
    }

    //---------- Parameter Size ----------

    pub fn check_parameter_size(&self) -> Result<(), Error> {
        for x in self.structures.iter() {
            x.check_parameter_size(self)?;
        }
        for x in self.classes.iter() {
            x.check_parameter_size(self)?;
        }
        for x in self.variables.iter() {
            x.check_parameter_size(self)?;
        }
        for x in self.functions.iter() {
            x.check_parameter_size(self)?;
        }
        for x in self.constraints.iter() {
            x.check_parameter_size(self)?;
        }
        self.search.check_parameter_size(self)?;
        Ok(())
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self) -> Result<(), Error> {
        for x in self.structures.iter() {
            x.check_bounded(self)?;
        }
        for x in self.classes.iter() {
            x.check_bounded(self)?;
        }
        for x in self.functions.iter() {
            x.check_bounded(self)?;
        }
        Ok(())
    }

    //---------- Typing ----------

    pub fn check_type(&self) -> Result<(), Error> {
        for x in self.structures.iter() {
            x.check_type(self)?;
        }
        for x in self.classes.iter() {
            x.check_type(self)?;
        }
        for x in self.variables.iter() {
            x.check_type(self)?;
        }
        for x in self.functions.iter() {
            x.check_type(self)?;
        }
        for x in self.constraints.iter() {
            x.check_type(self)?;
        }
        self.search.check_type(self)?;
        Ok(())
    }

    //---------- Empty ----------

    pub fn check_empty(&self) -> Result<(), Error> {
        for x in self.structures.iter() {
            if x.is_empty(self) {
                return Err(Error::Empty {
                    name: x.name().to_string(),
                    category: "Structure".to_string(),
                });
            }
        }
        for x in self.classes.iter() {
            if x.is_empty(self) {
                return Err(Error::Empty {
                    name: x.name().to_string(),
                    category: "Class".to_string(),
                });
            }
        }
        Ok(())
    }

    //---------- Cycle ----------

    pub fn check_cycle(&self) -> Result<(), Error> {
        for x in self.classes.iter() {
            x.check_cycle(self)?;
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
            write!(f, "// {:?}\n", x.id())?;
            write!(f, "{}\n", x.to_lang(self))?;
            let instances = x.instances(self);
            if let Some((first, others)) = instances.split_first() {
                write!(f, "inst {}", self.get(*first).unwrap().name())?;
                for i in others.iter() {
                    write!(f, ", {}", self.get(*i).unwrap().name())?;
                }
                write!(f, ": {}\n", x.name())?;
            }
        }
        for x in self.classes.iter() {
            write!(f, "// {:?}\n", x.id())?;
            write!(f, "{}\n", x.to_lang(self))?;
            let instances = x.instances(self);
            if let Some((first, others)) = instances.split_first() {
                write!(f, "inst {}", self.get(*first).unwrap().name())?;
                for i in others.iter() {
                    write!(f, ", {}", self.get(*i).unwrap().name())?;
                }
                write!(f, ": {}\n", x.name())?;
            }
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
        write!(f, "{}\n", self.search.to_lang(self))?;
        Ok(())
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<StructureId, Structure> for Problem {
    fn get(&self, id: StructureId) -> Option<&Structure> {
        self.get_structure(id)
    }
}
impl GetFromId<AttributeId<StructureId>, Attribute<StructureId>> for Problem {
    fn get(&self, id: AttributeId<StructureId>) -> Option<&Attribute<StructureId>> {
        let AttributeId(structure_id, _) = id;
        if let Some(structure) = self.get(structure_id) {
            structure.get(id)
        } else {
            None
        }
    }
}
impl GetFromId<MethodId<StructureId>, Method<StructureId>> for Problem {
    fn get(&self, id: MethodId<StructureId>) -> Option<&Method<StructureId>> {
        let MethodId(structure_id, _) = id;
        if let Some(structure) = self.get(structure_id) {
            structure.get(id)
        } else {
            None
        }
    }
}

impl GetFromId<ClassId, Class> for Problem {
    fn get(&self, id: ClassId) -> Option<&Class> {
        self.get_class(id)
    }
}
impl GetFromId<AttributeId<ClassId>, Attribute<ClassId>> for Problem {
    fn get(&self, id: AttributeId<ClassId>) -> Option<&Attribute<ClassId>> {
        let AttributeId(class_id, _) = id;
        if let Some(class) = self.get(class_id) {
            class.get(id)
        } else {
            None
        }
    }
}
impl GetFromId<MethodId<ClassId>, Method<ClassId>> for Problem {
    fn get(&self, id: MethodId<ClassId>) -> Option<&Method<ClassId>> {
        let MethodId(class_id, _) = id;
        if let Some(class) = self.get(class_id) {
            class.get(id)
        } else {
            None
        }
    }
}

impl GetFromId<InstanceId, Instance> for Problem {
    fn get(&self, id: InstanceId) -> Option<&Instance> {
        self.get_instance(id)
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

impl GetFromId<ConstraintId, Constraint> for Problem {
    fn get(&self, id: ConstraintId) -> Option<&Constraint> {
        self.get_constraint(id)
    }
}
