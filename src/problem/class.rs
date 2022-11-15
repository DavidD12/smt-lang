use super::*;
use crate::parser::Position;

pub enum ClassElement {
    Attribute(Attribute<ClassId>),
    Method(Method<ClassId>),
}

//------------------------- Id -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ClassId(pub usize);

impl Id for ClassId {
    fn empty() -> Self {
        Self(0)
    }
}

//------------------------- Class -------------------------

#[derive(Clone)]
pub struct Class {
    id: ClassId,
    name: String,
    typ: Type,
    extends: Option<Type>,
    attributes: Vec<Attribute<ClassId>>,
    methods: Vec<Method<ClassId>>,
    position: Option<Position>,
}

impl Class {
    pub fn new<S: Into<String>>(
        name: S,
        extends: Option<Type>,
        position: Option<Position>,
    ) -> Self {
        let id = ClassId::empty();
        let name = name.into();
        Self {
            id,
            name,
            typ: Type::Class(id),
            extends,
            attributes: vec![],
            methods: vec![],
            position,
        }
    }

    //---------- Extends ----------

    pub fn extends(&self) -> &Option<Type> {
        &self.extends
    }

    pub fn super_class(&self) -> Option<ClassId> {
        if let Some(t) = &self.extends {
            match t {
                Type::Class(c) => Some(*c),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn direct_sub_classes(&self, problem: &Problem) -> Vec<ClassId> {
        problem
            .classes()
            .iter()
            .filter(|c| c.super_class() == Some(self.id()))
            .map(|c| c.id())
            .collect()
    }

    pub fn super_classes(&self, problem: &Problem) -> Vec<ClassId> {
        if let Some(c) = self.super_class() {
            let mut v = Vec::new();
            let mut todo = vec![c];

            while !todo.is_empty() {
                let c = todo.pop().unwrap();
                if !v.contains(&c) {
                    v.push(c);
                    if let Some(s) = problem.get(c).unwrap().super_class() {
                        todo.push(s);
                    }
                }
            }
            v
        } else {
            vec![]
        }
    }

    pub fn sub_classes(&self, problem: &Problem) -> Vec<ClassId> {
        problem
            .classes()
            .iter()
            .filter(|c| c.super_classes(problem).contains(&self.id()))
            .map(|c| c.id())
            .collect()
    }

    pub fn common_class(&self, problem: &Problem, other: ClassId) -> Option<ClassId> {
        let mut l1 = self.super_classes(problem);
        l1.push(self.id());
        let mut l2 = problem.get(other).unwrap().super_classes(problem);
        l2.push(other);
        //
        let mut res = None;
        for c in l1.iter() {
            if !l2.contains(c) {
                return res;
            }
            res = Some(*c);
        }
        res
    }

    //---------- Attribute ----------

    pub fn add_attribute(&mut self, mut attribute: Attribute<ClassId>) -> AttributeId<ClassId> {
        let id = AttributeId(self.id(), self.attributes.len());
        attribute.set_id(id);
        self.attributes.push(attribute);
        id
    }

    pub fn get_attribute(&self, id: AttributeId<ClassId>) -> Option<&Attribute<ClassId>> {
        let AttributeId(structure_id, n) = id;
        if self.id != structure_id {
            None
        } else {
            self.attributes.get(n)
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute<ClassId>> {
        &self.attributes
    }

    pub fn all_attributes(&self, problem: &Problem) -> Vec<AttributeId<ClassId>> {
        let mut classes = self.super_classes(problem).clone();
        classes.push(self.id());
        let mut v = Vec::new();
        for id in classes.iter() {
            let c = problem.get(*id).unwrap();
            v.extend(c.attributes().iter().map(|a| a.id()));
        }
        v
    }

    pub fn find_attribute(&self, name: &str) -> Option<&Attribute<ClassId>> {
        self.attributes.iter().find(|x| x.name() == name)
    }

    pub fn find_all_attribute(&self, problem: &Problem, name: &str) -> Option<Attribute<ClassId>> {
        for x in self.attributes.iter() {
            if x.name() == name {
                return Some(x.clone());
            }
        }
        for c in self.super_classes(problem).iter() {
            let c = problem.get(*c).unwrap();
            for x in c.attributes.iter() {
                if x.name() == name {
                    return Some(x.clone());
                }
            }
        }
        None
    }

    //---------- Method ----------

    pub fn add_method(&mut self, mut method: Method<ClassId>) -> MethodId<ClassId> {
        let id = MethodId(self.id(), self.methods.len());
        method.set_id(id);
        self.methods.push(method);
        id
    }

    pub fn get_method(&self, id: MethodId<ClassId>) -> Option<&Method<ClassId>> {
        let MethodId(class_id, n) = id;
        if self.id != class_id {
            None
        } else {
            self.methods.get(n)
        }
    }

    pub fn methods(&self) -> &Vec<Method<ClassId>> {
        &self.methods
    }

    pub fn all_methods(&self, problem: &Problem) -> Vec<MethodId<ClassId>> {
        let mut classes = self.super_classes(problem).clone();
        classes.push(self.id());
        let mut v = Vec::new();
        for id in classes.iter() {
            let c = problem.get(*id).unwrap();
            v.extend(c.methods().iter().map(|a| a.id()));
        }
        v
    }

    pub fn find_method(&self, name: &str) -> Option<&Method<ClassId>> {
        self.methods.iter().find(|x| x.name() == name)
    }

    pub fn find_all_method(&self, problem: &Problem, name: &str) -> Option<Method<ClassId>> {
        for x in self.methods.iter() {
            if x.name() == name {
                return Some(x.clone());
            }
        }
        for c in self.super_classes(problem).iter() {
            let c = problem.get(*c).unwrap();
            for x in c.methods.iter() {
                if x.name() == name {
                    return Some(x.clone());
                }
            }
        }
        None
    }

    //---------- Instance ----------

    pub fn instances(&self, problem: &Problem) -> Vec<InstanceId> {
        let mut v = Vec::new();
        for inst in problem.instances().iter() {
            match inst.typ() {
                Type::Class(id) => {
                    if *id == self.id() {
                        v.push(inst.id());
                    }
                }
                _ => {}
            }
        }
        v
    }

    pub fn is_empty(&self, problem: &Problem) -> bool {
        self.all_instances(problem).is_empty()
    }

    pub fn all_instances(&self, problem: &Problem) -> Vec<InstanceId> {
        let mut v = self.instances(problem).clone();
        for c in self.sub_classes(problem).iter() {
            v.extend(problem.get(*c).unwrap().instances(problem));
        }
        v
    }

    //---------- Duplicate ----------

    pub fn local_naming(&self) -> Vec<Naming> {
        let mut v = vec![];
        v.extend(self.attributes.iter().map(|x| x.naming()));
        v.extend(self.methods.iter().map(|x| x.naming()));
        v
    }

    pub fn duplicate(&self) -> Result<(), Error> {
        check_duplicate(self.local_naming())?;
        for x in self.methods.iter() {
            x.duplicate()?;
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_expr(&self, problem: &Problem, entries: &Entries) -> Result<Class, Error> {
        // Attribute
        let mut attributes = Vec::new();
        for x in self.attributes.iter() {
            let a = x.resolve_expr(problem, &entries)?;
            attributes.push(a);
        }
        // Methods
        let mut methods = Vec::new();
        for x in self.methods.iter() {
            let m = x.resolve_expr(problem, &entries)?;
            methods.push(m);
        }
        //
        Ok(Class {
            id: self.id,
            name: self.name.clone(),
            typ: self.typ.clone(),
            extends: self.extends.clone(),
            attributes,
            methods,
            position: self.position.clone(),
        })
    }

    //---------- Parameter Size ----------

    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.attributes.iter() {
            x.check_parameter_size(problem)?;
        }
        for x in self.methods.iter() {
            x.check_parameter_size(problem)?;
        }
        Ok(())
    }

    //---------- Bounded ----------

    pub fn check_bounded(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.methods.iter() {
            x.check_bounded(problem)?;
        }
        Ok(())
    }

    //---------- Typing ----------

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.attributes.iter() {
            x.check_type(problem)?;
        }
        for x in self.methods.iter() {
            x.check_type(problem)?;
        }
        Ok(())
    }
}

//------------------------- Postion -------------------------

impl WithPosition for Class {
    fn position(&self) -> &Option<Position> {
        &self.position
    }
}

//------------------------- Named -------------------------

impl Named<ClassId> for Class {
    fn id(&self) -> ClassId {
        self.id
    }

    fn set_id(&mut self, id: ClassId) {
        self.id = id;
        self.typ = Type::Class(id);
        for (n, x) in self.attributes.iter_mut().enumerate() {
            let id = AttributeId(id, n);
            x.set_id(id);
        }
        for (n, x) in self.methods.iter_mut().enumerate() {
            let id = MethodId(id, n);
            x.set_id(id);
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//------------------------- With Type -------------------------

impl WithType for Class {
    fn typ(&self) -> &Type {
        &self.typ
    }

    fn set_type(&mut self, _: Type) {}

    fn resolve_type_children(&mut self, entries: &TypeEntries) -> Result<(), Error> {
        if let Some(c) = &self.extends {
            let c = c.resolve_type(entries)?;
            self.extends = Some(c);
        }
        for x in self.attributes.iter_mut() {
            x.resolve_type(&entries)?;
        }
        for x in self.methods.iter_mut() {
            x.resolve_type(&entries)?;
        }
        Ok(())
    }

    fn check_interval_children(&self, problem: &Problem) -> Result<(), Error> {
        for x in self.attributes.iter() {
            x.check_interval(problem)?;
        }
        for x in self.methods.iter() {
            x.check_interval(problem)?;
        }
        Ok(())
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Class {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = format!("class {}", self.name());
        // Exptends
        if let Some(c) = &self.extends {
            s.push_str(&format!(" extends {}", c.to_lang(problem)));
        }
        s.push_str(" {\n");
        // Attribute
        for x in self.attributes.iter() {
            s.push_str(&format!("{}\n", &x.to_lang(problem)));
        }
        // Method
        for x in self.methods.iter() {
            s.push_str(&format!("{}", &x.to_lang(problem)));
            s.push_str(&format!("// {:?}\n", x.id()));
        }
        //
        s.push_str("}\n");
        s
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<AttributeId<ClassId>, Attribute<ClassId>> for Class {
    fn get(&self, id: AttributeId<ClassId>) -> Option<&Attribute<ClassId>> {
        self.get_attribute(id)
    }
}

impl GetFromId<MethodId<ClassId>, Method<ClassId>> for Class {
    fn get(&self, id: MethodId<ClassId>) -> Option<&Method<ClassId>> {
        self.get_method(id)
    }
}
