use super::*;
use crate::error::Error;
use crate::parser::Position;
use fraction::Fraction;

use super::{Named, ToLang, VariableId};

//-------------------------------------------------- Bin Operator --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PreOp {
    Not,
    Minus,
}

impl std::fmt::Display for PreOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not => write!(f, "not"),
            Self::Minus => write!(f, "-"),
        }
    }
}

//-------------------------------------------------- Bin Operator --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BinOp {
    Eq,
    Ne,
    Lt,
    Le,
    Ge,
    Gt,
    //
    And,
    Or,
    Implies,
    //
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "="),
            Self::Ne => write!(f, "/="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Ge => write!(f, ">="),
            Self::Gt => write!(f, ">"),
            //
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Implies => write!(f, "=>"),
            //
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

//-------------------------------------------------- Expr --------------------------------------------------

#[derive(Clone, Debug)]
pub enum Expr {
    BoolValue(bool, Option<Position>),
    IntValue(isize, Option<Position>),
    RealValue(Fraction, Option<Position>),
    //
    Prefix(PreOp, Box<Expr>, Option<Position>),
    //
    Binary(Box<Expr>, BinOp, Box<Expr>, Option<Position>),
    //
    Variable(VariableId, Option<Position>),
    FunParam(ParameterId<FunctionId>, Option<Position>),
    FunctionCall(FunctionId, Vec<Expr>, Option<Position>),
    //
    Instance(InstanceId, Option<Position>),
    // Structure
    StrucSelf(StructureId, Option<Position>),
    StrucAttribute(Box<Expr>, AttributeId<StructureId>, Option<Position>),
    StrucMetCall(
        Box<Expr>,
        MethodId<StructureId>,
        Vec<Expr>,
        Option<Position>,
    ),
    StrucMetParam(ParameterId<MethodId<StructureId>>, Option<Position>),
    // Class
    ClassSelf(ClassId, Option<Position>),
    ClassAttribute(Box<Expr>, AttributeId<ClassId>, Option<Position>),
    ClassMetCall(Box<Expr>, MethodId<ClassId>, Vec<Expr>, Option<Position>),
    ClassMetParam(ParameterId<MethodId<ClassId>>, Option<Position>),
    AsClass(Box<Expr>, ClassId),
    //
    // Forall(String, Type, Box<Expr>, Option<Position>),
    //
    Unresolved(String, Option<Position>),
    UnresolvedFunCall(String, Vec<Expr>, Option<Position>),
    UnresolvedAttribute(Box<Expr>, String, Option<Position>),
    UnresolvedMethCall(Box<Expr>, String, Vec<Expr>, Option<Position>),
}

impl Expr {
    pub fn position(&self) -> Option<Position> {
        match self {
            Expr::BoolValue(_, p) => p.clone(),
            Expr::IntValue(_, p) => p.clone(),
            Expr::RealValue(_, p) => p.clone(),
            Expr::Prefix(_, _, p) => p.clone(),
            Expr::Binary(_, _, _, p) => p.clone(),
            Expr::FunctionCall(_, _, p) => p.clone(),
            Expr::Variable(_, p) => p.clone(),
            Expr::FunParam(_, p) => p.clone(),
            Expr::Instance(_, p) => p.clone(),
            Expr::StrucSelf(_, p) => p.clone(),
            Expr::StrucAttribute(_, _, p) => p.clone(),
            Expr::StrucMetCall(_, _, _, p) => p.clone(),
            Expr::StrucMetParam(_, p) => p.clone(),
            Expr::ClassSelf(_, p) => p.clone(),
            Expr::ClassAttribute(_, _, p) => p.clone(),
            Expr::ClassMetCall(_, _, _, p) => p.clone(),
            Expr::ClassMetParam(_, p) => p.clone(),
            Expr::AsClass(_, _) => None,
            // Expr::Forall(_, _, _, p) => p.clone(),
            Expr::Unresolved(_, p) => p.clone(),
            Expr::UnresolvedFunCall(_, _, p) => p.clone(),
            Expr::UnresolvedAttribute(_, _, p) => p.clone(),
            Expr::UnresolvedMethCall(_, _, _, p) => p.clone(),
        }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::BoolValue(x, _), Expr::BoolValue(y, _)) => x == y,
            (Expr::IntValue(x, _), Expr::IntValue(y, _)) => x == y,
            (Expr::RealValue(x, _), Expr::RealValue(y, _)) => x == y,
            (Expr::Prefix(o1, e1, _), Expr::Prefix(o2, e2, _)) => o1 == o2 && e1.is_same(e2),
            (Expr::Binary(l1, o1, r1, _), Expr::Binary(l2, o2, r2, _)) => {
                o1 == o2 && l1.is_same(l2) && r1.is_same(r2)
            }
            (Expr::FunctionCall(i1, p1, _), Expr::FunctionCall(i2, p2, _)) => {
                i1 == i2 && Self::all_same(p1, p2)
            }
            (Expr::Variable(i1, _), Expr::Variable(i2, _)) => i1 == i2,
            (Expr::FunParam(i1, _), Expr::FunParam(i2, _)) => i1 == i2,
            (Expr::Instance(i1, _), Expr::Instance(i2, _)) => i1 == i2,
            (Expr::StrucSelf(i1, _), Expr::StrucSelf(i2, _)) => i1 == i2,
            (Expr::StrucAttribute(_, i1, _), Expr::StrucAttribute(_, i2, _)) => i1 == i2,
            (Expr::StrucMetCall(e1, i1, a1, _), Expr::StrucMetCall(e2, i2, a2, _)) => {
                e1.is_same(e2) && i1 == i2 && Self::all_same(a1, a2)
            }
            (Expr::StrucMetParam(i1, _), Expr::StrucMetParam(i2, _)) => i1 == i2,
            (Expr::AsClass(e1, i1), Expr::AsClass(e2, i2)) => i1 == i2 && e1.is_same(e2),
            _ => false,
        }
    }

    pub fn all_same(v1: &Vec<Expr>, v2: &Vec<Expr>) -> bool {
        v1.iter().zip(v2.iter()).all(|(x, y)| x.is_same(y))
    }

    pub fn resolve(&self, problem: &Problem, entries: &Entries) -> Result<Expr, Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(self.clone()),
            Expr::IntValue(_, _) => Ok(self.clone()),
            Expr::RealValue(_, _) => Ok(self.clone()),
            //
            Expr::Prefix(op, kid, position) => {
                let kid = kid.resolve(problem, entries)?;
                Ok(Self::Prefix(*op, Box::new(kid), position.clone()))
            }
            Expr::Binary(left, op, right, position) => {
                let left = left.resolve(problem, entries)?;
                let right = right.resolve(problem, entries)?;
                Ok(Self::Binary(
                    Box::new(left),
                    *op,
                    Box::new(right),
                    position.clone(),
                ))
            }
            //
            Expr::FunctionCall(id, parameters, position) => {
                let mut v: Vec<Expr> = vec![];
                for p in parameters.iter() {
                    v.push(p.resolve(problem, entries)?);
                }
                Ok(Self::FunctionCall(*id, v, position.clone()))
            }
            //
            Expr::Variable(_, _) => Ok(self.clone()),
            Expr::FunParam(_, _) => Ok(self.clone()),
            Expr::Instance(_, _) => Ok(self.clone()),
            Expr::StrucSelf(_, _) => Ok(self.clone()),
            Expr::StrucAttribute(e, id, pos) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::StrucAttribute(Box::new(e), *id, pos.clone()))
            }
            Expr::StrucMetCall(e, id, args, pos) => {
                let e = e.resolve(problem, entries)?;
                let mut v: Vec<Expr> = vec![];
                for p in args.iter() {
                    v.push(p.resolve(problem, entries)?);
                }

                Ok(Expr::StrucMetCall(Box::new(e), *id, v, pos.clone()))
            }
            Expr::StrucMetParam(_, _) => Ok(self.clone()),
            //
            Expr::ClassSelf(_, _) => Ok(self.clone()),
            Expr::ClassAttribute(e, id, pos) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::ClassAttribute(Box::new(e), *id, pos.clone()))
            }
            Expr::ClassMetCall(e, id, args, pos) => {
                let e = e.resolve(problem, entries)?;
                let mut v: Vec<Expr> = vec![];
                for p in args.iter() {
                    v.push(p.resolve(problem, entries)?);
                }

                Ok(Expr::ClassMetCall(Box::new(e), *id, v, pos.clone()))
            }
            Expr::ClassMetParam(_, _) => Ok(self.clone()),
            Expr::AsClass(e, id) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::AsClass(Box::new(e), *id))
            }
            //
            Expr::Unresolved(name, position) => match entries.get(&name) {
                Some(entry) => match entry.typ() {
                    EntryType::Variable(id) => Ok(Self::Variable(id, position.clone())),
                    EntryType::FunParam(id) => Ok(Self::FunParam(id, position.clone())),
                    EntryType::Instance(id) => Ok(Expr::Instance(id, position.clone())),
                    EntryType::StrucSelf(id) => Ok(Expr::StrucSelf(id, position.clone())),
                    EntryType::StrucMetParam(id) => Ok(Self::StrucMetParam(id, position.clone())),
                    EntryType::ClassSelf(id) => Ok(Expr::ClassSelf(id, position.clone())),
                    EntryType::ClassMetParam(id) => Ok(Self::ClassMetParam(id, position.clone())),
                },
                None => Err(Error::Resolve {
                    category: "identifier".to_string(),
                    name: name.clone(),
                    position: position.clone(),
                }),
            },
            Expr::UnresolvedFunCall(name, params, position) => {
                let mut v: Vec<Expr> = vec![];
                for p in params.iter() {
                    v.push(p.resolve(problem, entries)?);
                }
                if let Some(function) = problem.find_function(name) {
                    return Ok(Expr::FunctionCall(function.id(), v, position.clone()));
                }
                Err(Error::Resolve {
                    category: "function".to_string(),
                    name: name.clone(),
                    position: position.clone(),
                })
            }
            Expr::UnresolvedAttribute(e, name, pos) => {
                let e = e.resolve(problem, entries)?;
                let t = e.typ(problem);
                if let Type::Structure(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_attribute(name) {
                        Ok(Expr::StrucAttribute(Box::new(e), a.id(), pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "attibute for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else if let Type::Class(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_all_attribute(problem, name) {
                        Ok(Expr::ClassAttribute(Box::new(e), a.id(), pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "attibute for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else {
                    panic!()
                }
            }
            Expr::UnresolvedMethCall(e, name, args, pos) => {
                let e = e.resolve(problem, entries)?;
                let mut v: Vec<Expr> = vec![];
                for p in args.iter() {
                    v.push(p.resolve(problem, entries)?);
                }
                let t = e.typ(problem);
                if let Type::Structure(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_method(name) {
                        Ok(Expr::StrucMetCall(Box::new(e), a.id(), v, pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "method for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else if let Type::Class(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_all_method(problem, name) {
                        Ok(Expr::ClassMetCall(Box::new(e), a.id(), v, pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "method for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else {
                    panic!()
                }
            }
        }
    }

    pub fn typ(&self, problem: &Problem) -> Type {
        match self {
            Expr::BoolValue(_, _) => Type::Bool,
            Expr::IntValue(value, _) => Type::Interval(*value, *value),
            Expr::RealValue(_, _) => Type::Real,
            Expr::Prefix(op, e, _) => match op {
                PreOp::Not => Type::Bool,
                PreOp::Minus => match e.typ(problem) {
                    Type::Int => Type::Int,
                    Type::Real => Type::Real,
                    Type::Interval(min, max) => Type::Interval(-max, -min),
                    _ => Type::Undefined,
                },
            },
            Expr::Binary(left, op, right, _) => match op {
                BinOp::Eq => Type::Bool,
                BinOp::Ne => Type::Bool,
                BinOp::Lt => Type::Bool,
                BinOp::Le => Type::Bool,
                BinOp::Ge => Type::Bool,
                BinOp::Gt => Type::Bool,
                BinOp::And => Type::Bool,
                BinOp::Or => Type::Bool,
                BinOp::Implies => Type::Bool,
                //
                BinOp::Add => match (left.typ(problem), right.typ(problem)) {
                    (Type::Int, _) => Type::Int,
                    (Type::Real, _) => Type::Real,
                    (Type::Interval(_, _), Type::Int) => Type::Int,
                    (Type::Interval(min1, max1), Type::Interval(min2, max2)) => {
                        Type::Interval(min1 + min2, max1 + max2)
                    }
                    _ => Type::Undefined,
                },
                BinOp::Sub => match (left.typ(problem), right.typ(problem)) {
                    (Type::Int, _) => Type::Int,
                    (Type::Real, _) => Type::Real,
                    (Type::Interval(_, _), Type::Int) => Type::Int,
                    (Type::Interval(min1, max1), Type::Interval(min2, max2)) => {
                        Type::Interval(min1 - min2, max1 - max2)
                    }
                    _ => Type::Undefined,
                },
                BinOp::Mul => match (left.typ(problem), right.typ(problem)) {
                    (Type::Int, _) => Type::Int,
                    (Type::Real, _) => Type::Real,
                    (Type::Interval(_, _), Type::Int) => Type::Int,
                    (Type::Interval(min1, max1), Type::Interval(min2, max2)) => {
                        Type::Interval(min1 * min2, max1 * max2)
                    }
                    _ => Type::Undefined,
                },
                BinOp::Div => Type::Real,
            },
            Expr::FunctionCall(id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::Instance(id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::Variable(id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::FunParam(id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::StrucSelf(id, _) => Type::Structure(*id),
            Expr::StrucAttribute(_, id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::StrucMetCall(_, id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::StrucMetParam(id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::ClassSelf(id, _) => Type::Class(*id),
            Expr::ClassAttribute(_, id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::ClassMetCall(_, id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::ClassMetParam(id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::AsClass(_, id) => Type::Class(*id),
            Expr::Unresolved(_, _) => Type::Undefined,
            Expr::UnresolvedFunCall(_, _, _) => Type::Undefined,
            Expr::UnresolvedAttribute(_, _, _) => Type::Undefined,
            Expr::UnresolvedMethCall(_, _, _, _) => Type::Undefined,
        }
    }

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(()),
            Expr::IntValue(_, _) => Ok(()),
            Expr::RealValue(_, _) => Ok(()),
            Expr::Prefix(op, e, _) => {
                e.check_type(problem)?;
                let e_type = e.typ(problem);
                match op {
                    PreOp::Not => check_type_bool(e, &e_type),
                    PreOp::Minus => check_type_number(e, &e_type),
                }
            }
            Expr::Binary(l, op, r, _) => {
                l.check_type(problem)?;
                r.check_type(problem)?;
                let l_type = l.typ(problem);
                let r_type = r.typ(problem);
                // Eq/Ne
                if [BinOp::Eq, BinOp::Ne].contains(op) {
                    check_compatible_type(problem, &l_type, r, &r_type)
                }
                // Compare
                else if [BinOp::Lt, BinOp::Le, BinOp::Ge, BinOp::Gt].contains(op) {
                    check_type_number(l, &l_type)?;
                    check_type_number(r, &r_type)?;
                    check_compatible_type(problem, &l_type, r, &r_type)
                }
                // Bool
                else if [BinOp::And, BinOp::Or, BinOp::Implies].contains(op) {
                    check_type_bool(l, &l_type)?;
                    check_type_bool(r, &r_type)
                }
                // Arith
                else if [BinOp::Add, BinOp::Sub, BinOp::Mul].contains(op) {
                    check_type_number(l, &l_type)?;
                    check_type_number(r, &r_type)?;
                    check_compatible_type(problem, &l_type, r, &r_type)
                }
                // Div
                else if *op == BinOp::Div {
                    check_type_number(l, &l_type)?;
                    check_type_number(r, &r_type)
                } else {
                    panic!("undefined")
                }
            }
            Expr::FunctionCall(id, params, _) => {
                for p in params.iter() {
                    p.check_type(problem)?;
                }
                let fun = problem.get(*id).unwrap();
                for (p, e) in fun.parameters().iter().zip(params.iter()) {
                    check_subtype_type(problem, &p.typ(), e, &e.typ(problem))?;
                }
                Ok(())
            }
            Expr::Instance(_, _) => Ok(()),
            Expr::Variable(_, _) => Ok(()),
            Expr::FunParam(_, _) => Ok(()),
            Expr::StrucSelf(_, _) => Ok(()),
            Expr::StrucAttribute(e, id, _) => {
                let et = e.typ(problem);
                let AttributeId(structure_id, _) = id;
                let st = problem.get(*structure_id).unwrap().typ();
                check_subtype_type(problem, &st, e, &et)
            }
            Expr::StrucMetCall(e, id, args, _) => {
                let et = e.typ(problem);
                let MethodId(structure_id, _) = id;
                let st = problem.get(*structure_id).unwrap().typ();
                let meth = problem.get(*id).unwrap();
                check_subtype_type(problem, &st, e, &et)?;
                for (p, e) in meth.parameters().iter().zip(args.iter()) {
                    check_subtype_type(problem, &p.typ(), e, &e.typ(problem))?;
                }
                Ok(())
            }
            Expr::StrucMetParam(_, _) => Ok(()),
            Expr::ClassSelf(_, _) => Ok(()),
            Expr::ClassAttribute(e, id, _) => {
                let et = e.typ(problem);
                let AttributeId(class_id, _) = id;
                let st = problem.get(*class_id).unwrap().typ();
                check_subtype_type(problem, &st, e, &et)
            }
            Expr::ClassMetCall(e, id, args, _) => {
                let et = e.typ(problem);
                let MethodId(class_id, _) = id;
                let st = problem.get(*class_id).unwrap().typ();
                let meth = problem.get(*id).unwrap();
                check_subtype_type(problem, &st, e, &et)?;
                for (p, e) in meth.parameters().iter().zip(args.iter()) {
                    check_subtype_type(problem, &p.typ(), e, &e.typ(problem))?;
                }
                Ok(())
            }
            Expr::ClassMetParam(_, _) => Ok(()),
            Expr::AsClass(e, id) => {
                check_subtype_type(problem, &Type::Class(*id), e, &e.typ(problem))
            }
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
            Expr::UnresolvedAttribute(_, _, _) => panic!(),
            Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
        }
    }

    //---------- Parameter Size ----------

    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(()),
            Expr::IntValue(_, _) => Ok(()),
            Expr::RealValue(_, _) => Ok(()),
            Expr::Prefix(_, e, _) => e.check_type(problem),
            Expr::Binary(l, _, r, _) => {
                l.check_parameter_size(problem)?;
                r.check_parameter_size(problem)
            }
            Expr::FunctionCall(id, v, _) => {
                for x in v.iter() {
                    x.check_parameter_size(problem)?;
                }
                let fun = problem.get(*id).unwrap();
                if fun.parameters().len() == v.len() {
                    Ok(())
                } else {
                    Err(Error::Parameter {
                        expr: self.clone(),
                        size: v.len(),
                        expected: fun.parameters().len(),
                    })
                }
            }
            Expr::Instance(_, _) => Ok(()),
            Expr::Variable(_, _) => Ok(()),
            Expr::FunParam(_, _) => Ok(()),
            Expr::StrucSelf(_, _) => Ok(()),
            Expr::StrucAttribute(e, _, _) => e.check_parameter_size(problem),
            Expr::StrucMetCall(e, id, v, _) => {
                e.check_parameter_size(problem)?;
                for x in v.iter() {
                    x.check_parameter_size(problem)?;
                }
                let meth = problem.get(*id).unwrap();
                if meth.parameters().len() == v.len() {
                    Ok(())
                } else {
                    Err(Error::Parameter {
                        expr: self.clone(),
                        size: v.len(),
                        expected: meth.parameters().len(),
                    })
                }
            }
            Expr::StrucMetParam(_, _) => Ok(()),
            Expr::ClassSelf(_, _) => Ok(()),
            Expr::ClassAttribute(e, _, _) => e.check_parameter_size(problem),
            Expr::ClassMetCall(e, id, v, _) => {
                e.check_parameter_size(problem)?;
                for x in v.iter() {
                    x.check_parameter_size(problem)?;
                }
                let meth = problem.get(*id).unwrap();
                if meth.parameters().len() == v.len() {
                    Ok(())
                } else {
                    Err(Error::Parameter {
                        expr: self.clone(),
                        size: v.len(),
                        expected: meth.parameters().len(),
                    })
                }
            }
            Expr::ClassMetParam(_, _) => Ok(()),
            Expr::AsClass(e, _) => e.check_parameter_size(problem),
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
            Expr::UnresolvedAttribute(_, _, _) => panic!(),
            Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
        }
    }

    pub fn substitute(&self, old: &Expr, expr: &Expr) -> Expr {
        if self.is_same(old) {
            expr.clone()
        } else {
            match self {
                Expr::BoolValue(_, _) => self.clone(),
                Expr::IntValue(_, _) => self.clone(),
                Expr::RealValue(_, _) => self.clone(),
                Expr::Prefix(op, e, pos) => {
                    Expr::Prefix(*op, Box::new(e.substitute(old, expr)), pos.clone())
                }
                Expr::Binary(left, op, right, pos) => {
                    let left = Box::new(left.substitute(old, expr));
                    let right = Box::new(right.substitute(old, expr));
                    Expr::Binary(left, *op, right, pos.clone())
                }
                Expr::FunctionCall(id, params, pos) => {
                    let params = params.iter().map(|p| p.substitute(old, expr)).collect();
                    Expr::FunctionCall(*id, params, pos.clone())
                }
                Expr::Instance(_, _) => self.clone(),
                Expr::Variable(_, _) => self.clone(),
                Expr::FunParam(_, _) => self.clone(),
                Expr::StrucSelf(_, _) => self.clone(),
                Expr::StrucAttribute(e, id, pos) => {
                    Expr::StrucAttribute(Box::new(e.substitute(old, expr)), *id, pos.clone())
                }
                Expr::StrucMetCall(e, id, args, pos) => {
                    let e = e.substitute(old, expr);
                    let args = args.iter().map(|a| a.substitute(old, expr)).collect();
                    Expr::StrucMetCall(Box::new(e), *id, args, pos.clone())
                }
                Expr::StrucMetParam(_, _) => self.clone(),
                Expr::ClassSelf(_, _) => self.clone(),
                Expr::ClassAttribute(e, id, pos) => {
                    Expr::ClassAttribute(Box::new(e.substitute(old, expr)), *id, pos.clone())
                }
                Expr::ClassMetCall(e, id, args, pos) => {
                    let e = e.substitute(old, expr);
                    let args = args.iter().map(|a| a.substitute(old, expr)).collect();
                    Expr::ClassMetCall(Box::new(e), *id, args, pos.clone())
                }
                Expr::ClassMetParam(_, _) => self.clone(),
                Expr::AsClass(e, id) => Expr::AsClass(Box::new(e.substitute(old, expr)), *id),
                Expr::Unresolved(_, _) => self.clone(),
                Expr::UnresolvedFunCall(_, _, _) => panic!(),
                Expr::UnresolvedAttribute(_, _, _) => panic!(),
                Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
            }
        }
    }

    pub fn substitute_all(&self, all: Vec<(Expr, Expr)>) -> Expr {
        let mut expr = self.clone();
        for (o, e) in all.iter() {
            expr = expr.substitute(o, e);
        }
        expr
    }
}

pub fn check_type_bool(expr: &Expr, expr_type: &Type) -> Result<(), Error> {
    if expr_type.is_bool() {
        Ok(())
    } else {
        Err(Error::Type {
            expr: expr.clone(),
            typ: expr_type.clone(),
            expected: vec![Type::Bool],
        })
    }
}

pub fn check_type_number(expr: &Expr, expr_type: &Type) -> Result<(), Error> {
    if expr_type.is_number() {
        Ok(())
    } else {
        Err(Error::Type {
            expr: expr.clone(),
            typ: expr_type.clone(),
            expected: vec![Type::Int, Type::Real],
        })
    }
}

pub fn check_compatible_type(
    problem: &Problem,
    left_type: &Type,
    right: &Expr,
    right_type: &Type,
) -> Result<(), Error> {
    if right_type.is_compatible_with(problem, left_type) {
        Ok(())
    } else {
        Err(Error::Type {
            expr: right.clone(),
            typ: right_type.clone(),
            expected: vec![left_type.clone()],
        })
    }
}

pub fn check_subtype_type(
    problem: &Problem,
    left_type: &Type,
    right: &Expr,
    right_type: &Type,
) -> Result<(), Error> {
    if right_type.is_subtype_of(problem, left_type) {
        Ok(())
    } else {
        Err(Error::Type {
            expr: right.clone(),
            typ: right_type.clone(),
            expected: vec![left_type.clone()],
        })
    }
}

//------------------------- To Lang -------------------------

impl ToLang for Expr {
    fn to_lang(&self, problem: &super::Problem) -> String {
        match self {
            Expr::BoolValue(value, _) => format!("{}", value),
            Expr::IntValue(value, _) => format!("{}", value),
            Expr::RealValue(value, _) => format!("{}", value),
            Expr::Prefix(op, kid, _) => format!("({} {}", op, kid.to_lang(problem)),
            Expr::Binary(left, op, right, _) => format!(
                "({} {} {})",
                left.to_lang(problem),
                op,
                right.to_lang(problem)
            ),
            Expr::FunctionCall(id, params, _) => {
                let fun = problem.get(*id).unwrap();
                let mut s = format!("{}(", fun.name());
                if let Some((first, others)) = params.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for p in others.iter() {
                        s.push_str(&format!(", {}", p.to_lang(problem)));
                    }
                }
                s.push_str(")");
                s
            }
            Expr::Instance(id, _) => problem.get(*id).unwrap().name().into(),
            Expr::Variable(id, _) => problem.get(*id).unwrap().name().into(),
            Expr::FunParam(id, _) => problem.get(*id).unwrap().name().into(),
            Expr::StrucSelf(_, _) => "self".to_string(),
            Expr::StrucAttribute(e, id, _) => {
                format!(
                    "({}.{})",
                    e.to_lang(problem),
                    problem.get(*id).unwrap().name()
                )
            }
            Expr::StrucMetCall(e, id, args, _) => {
                let name = problem.get(*id).unwrap().name();
                let mut s = format!("{}.{}(", e.to_lang(problem), name,);
                if let Some((first, others)) = args.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for p in others.iter() {
                        s.push_str(&format!(", {}", p.to_lang(problem)));
                    }
                }
                s.push_str(")");
                s
            }
            Expr::StrucMetParam(id, _) => problem.get(*id).unwrap().name().into(),
            Expr::ClassSelf(_, _) => "self".to_string(),
            Expr::ClassAttribute(e, id, _) => {
                format!(
                    "({}.{})",
                    e.to_lang(problem),
                    problem.get(*id).unwrap().name()
                )
            }
            Expr::ClassMetCall(e, id, args, _) => {
                let name = problem.get(*id).unwrap().name();
                let mut s = format!("{}.{}(", e.to_lang(problem), name,);
                if let Some((first, others)) = args.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for p in others.iter() {
                        s.push_str(&format!(", {}", p.to_lang(problem)));
                    }
                }
                s.push_str(")");
                s
            }
            Expr::ClassMetParam(id, _) => problem.get(*id).unwrap().name().into(),
            Expr::AsClass(e, id) => format!(
                "({} as {})",
                e.to_lang(problem),
                problem.get(*id).unwrap().name()
            ),
            Expr::Unresolved(name, _) => format!("{}?", name),
            Expr::UnresolvedFunCall(name, params, _) => {
                let mut s = format!("{}?(", name);
                if let Some((first, others)) = params.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for p in others.iter() {
                        s.push_str(&format!(", {}", p.to_lang(problem)));
                    }
                }
                s.push_str(")");
                s
            }
            Expr::UnresolvedAttribute(e, name, _) => format!("({}.{})?", e.to_lang(problem), name),
            Expr::UnresolvedMethCall(e, name, args, _) => {
                let mut s = format!("{}.{}(", e.to_lang(problem), name,);
                if let Some((first, others)) = args.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for p in others.iter() {
                        s.push_str(&format!(", {}", p.to_lang(problem)));
                    }
                }
                s.push_str(")");
                s
            }
        }
    }
}
