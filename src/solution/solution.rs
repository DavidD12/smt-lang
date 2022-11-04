use super::Value;
use super::*;
use crate::problem::*;
use crate::solve::Smt;
use std::collections::HashMap;

pub struct Solution {
    // Variable
    variables: HashMap<VariableId, Value>,
    // Function
    functions: HashMap<FunctionId, FunctionValue>,
}

impl Solution {
    pub fn new(smt: &Smt, model: &z3::Model) -> Self {
        let mut variables = HashMap::new();
        // Variables
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
            variables,
            functions,
        }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for Solution {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = "".to_string();
        // Variables
        for variable in problem.variables().iter() {
            let t = &variable.typ();
            let mut v = variable.clone();
            v.clear_expr();
            let value = self.variables.get(&v.id()).unwrap();
            s.push_str(&format!("{} = {}\n", v.to_lang(problem), value));
        }
        // Functions
        for function in problem.functions().iter() {
            let mut f = function.clone();
            f.clear_expr();
            let value = self.functions.get(&f.id()).unwrap();
            s.push_str(&format!("{} = {}\n", f.to_lang(problem), value));
        }
        //
        s
    }
}
