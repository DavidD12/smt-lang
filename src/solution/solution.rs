use crate::problem::*;
use crate::solve::Smt;
use fraction::Fraction;
use std::collections::HashMap;

pub struct Solution {
    bool_variables: HashMap<VariableId, bool>,
    int_variables: HashMap<VariableId, isize>,
    real_variables: HashMap<VariableId, Fraction>,
}

impl Solution {
    pub fn new(smt: &Smt, model: &z3::Model) -> Self {
        let mut bool_variables = HashMap::new();
        let mut int_variables = HashMap::new();
        let mut real_variables = HashMap::new();
        for variable in smt.problem().variables() {
            match variable.typ() {
                Type::Bool => {
                    let e = smt.bool_variable(variable.id());
                    let value = model.eval(e, true).unwrap().as_bool().unwrap();
                    bool_variables.insert(variable.id(), value);
                }
                Type::Int => {
                    let e = smt.int_variable(variable.id());
                    let value = model.eval(e, true).unwrap().as_i64().unwrap();
                    int_variables.insert(variable.id(), value as isize);
                }
                Type::Real => {
                    let e = smt.real_variable(variable.id());
                    let (num, den) = model.eval(e, true).unwrap().as_real().unwrap();
                    let value = if num >= 0 {
                        Fraction::new_generic(fraction::Sign::Plus, num, den).unwrap()
                    } else {
                        Fraction::new_generic(fraction::Sign::Minus, -num, den).unwrap()
                    };
                    real_variables.insert(variable.id(), value);
                }
                Type::Interval(_, _) => {
                    let e = smt.int_variable(variable.id());
                    let value = model.eval(e, true).unwrap().as_i64().unwrap();
                    int_variables.insert(variable.id(), value as isize);
                }
                Type::Undefined => panic!(),
            }
        }
        Self {
            bool_variables,
            int_variables,
            real_variables,
        }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for Solution {
    fn to_lang(&self, problem: &Problem) -> String {
        let mut s = "".to_string();
        for variable in problem.variables().iter() {
            match variable.typ() {
                Type::Bool => s.push_str(&format!(
                    "let {}: Bool = {}\n",
                    variable.name(),
                    self.bool_variables.get(&variable.id()).unwrap()
                )),
                Type::Int => s.push_str(&format!(
                    "let {}: Int = {}\n",
                    variable.name(),
                    self.int_variables.get(&variable.id()).unwrap()
                )),
                Type::Real => s.push_str(&format!(
                    "let {}: Real = {}\n",
                    variable.name(),
                    self.real_variables.get(&variable.id()).unwrap()
                )),
                Type::Interval(min, max) => s.push_str(&format!(
                    "let {}: {}..{} = {}\n",
                    variable.name(),
                    min,
                    max,
                    self.int_variables.get(&variable.id()).unwrap()
                )),
                Type::Undefined => panic!(),
            }
        }
        s
    }
}
