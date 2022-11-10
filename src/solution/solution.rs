use super::Value;
use super::*;
use crate::problem::*;
use crate::solve::Smt;
use std::collections::HashMap;

pub struct Solution {
    // Structure
    structures: HashMap<StructureId, StructureValue>,
    // Variable
    variables: HashMap<VariableId, Value>,
    // Function
    functions: HashMap<FunctionId, FunctionValue>,
}

impl Solution {
    pub fn new(smt: &Smt, model: &z3::Model) -> Self {
        // Structures
        let mut structures = HashMap::new();
        for structure in smt.problem().structures().iter() {
            if !structure.attributes().is_empty() || !structure.methods().is_empty() {
                let value = StructureValue::new(smt, model, structure.id());
                structures.insert(structure.id(), value);
            }
        }
        // Variables
        let mut variables = HashMap::new();
        for variable in smt.problem().variables().iter() {
            let value = Value::new(smt, model, &Expr::Variable(variable.id(), None));
            variables.insert(variable.id(), value);
        }

        // Functions
        let mut functions = HashMap::new();
        for function in smt.problem().functions().iter() {
            let value = FunctionValue::new(smt, model, function);
            functions.insert(function.id(), value);
        }
        //
        Self {
            structures,
            variables,
            functions,
        }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for Solution {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = "".to_string();
        // Structures
        for structure in self.structures.values() {
            s.push_str(&structure.to_lang(problem));
        }
        // Variables
        for variable in problem.variables().iter() {
            let mut v = variable.clone();
            v.clear_expr();
            let value = self.variables.get(&v.id()).unwrap();
            s.push_str(&format!(
                "{} = {}\n",
                v.to_lang(problem),
                value.to_lang(problem)
            ));
        }
        // Functions
        for function in problem.functions().iter() {
            let mut f = function.clone();
            f.clear_expr();
            let value = self.functions.get(&f.id()).unwrap();
            s.push_str(&format!(
                "{} = {}\n",
                f.to_lang(problem),
                value.to_lang(problem)
            ));
        }
        //
        s
    }
}
