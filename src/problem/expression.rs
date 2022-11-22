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
    Parameter(Parameter),
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
    // Class
    ClassSelf(ClassId, Option<Position>),
    ClassAttribute(Box<Expr>, AttributeId<ClassId>, Option<Position>),
    ClassMetCall(Box<Expr>, MethodId<ClassId>, Vec<Expr>, Option<Position>),
    //
    AsClass(Box<Expr>, ClassId),
    AsInterval(Box<Expr>, isize, isize, Option<Position>),
    AsInt(Box<Expr>, Option<Position>),
    AsReal(Box<Expr>, Option<Position>),
    //
    IfThenElse(
        Box<Expr>,
        Box<Expr>,
        Vec<(Expr, Expr)>,
        Box<Expr>,
        Option<Position>,
    ),
    //
    Forall(Vec<Parameter>, Box<Expr>, Option<Position>),
    Exists(Vec<Parameter>, Box<Expr>, Option<Position>),
    Sum(Vec<Parameter>, Box<Expr>, Option<Position>),
    Prod(Vec<Parameter>, Box<Expr>, Option<Position>),
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
            Expr::Parameter(p) => p.position().clone(),
            Expr::Instance(_, p) => p.clone(),
            Expr::StrucSelf(_, p) => p.clone(),
            Expr::StrucAttribute(_, _, p) => p.clone(),
            Expr::StrucMetCall(_, _, _, p) => p.clone(),
            Expr::ClassSelf(_, p) => p.clone(),
            Expr::ClassAttribute(_, _, p) => p.clone(),
            Expr::ClassMetCall(_, _, _, p) => p.clone(),
            Expr::AsClass(_, _) => None,
            Expr::AsInterval(_, _, _, p) => p.clone(),
            Expr::AsInt(_, p) => p.clone(),
            Expr::AsReal(_, p) => p.clone(),
            Expr::IfThenElse(_, _, _, _, p) => p.clone(),
            Expr::Forall(_, _, p) => p.clone(),
            Expr::Exists(_, _, p) => p.clone(),
            Expr::Sum(_, _, p) => p.clone(),
            Expr::Prod(_, _, p) => p.clone(),
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
            (Expr::Parameter(p1), Expr::Parameter(p2)) => p1.is_same(p2),
            (Expr::Instance(i1, _), Expr::Instance(i2, _)) => i1 == i2,
            (Expr::StrucSelf(i1, _), Expr::StrucSelf(i2, _)) => i1 == i2,
            (Expr::StrucAttribute(_, i1, _), Expr::StrucAttribute(_, i2, _)) => i1 == i2,
            (Expr::StrucMetCall(e1, i1, a1, _), Expr::StrucMetCall(e2, i2, a2, _)) => {
                e1.is_same(e2) && i1 == i2 && Self::all_same(a1, a2)
            }
            (Expr::ClassSelf(i1, _), Expr::ClassSelf(i2, _)) => i1 == i2,
            (Expr::ClassAttribute(_, i1, _), Expr::ClassAttribute(_, i2, _)) => i1 == i2,
            (Expr::ClassMetCall(e1, i1, a1, _), Expr::ClassMetCall(e2, i2, a2, _)) => {
                e1.is_same(e2) && i1 == i2 && Self::all_same(a1, a2)
            }
            (Expr::AsClass(e1, i1), Expr::AsClass(e2, i2)) => i1 == i2 && e1.is_same(e2),
            (Expr::AsInterval(e1, min1, max1, _), Expr::AsInterval(e2, min2, max2, _)) => {
                min1 == min2 && max1 == max2 && e1.is_same(e2)
            }
            (Expr::AsInt(e1, _), Expr::AsInt(e2, _)) => e1.is_same(&e2),
            (Expr::AsReal(e1, _), Expr::AsReal(e2, _)) => e1.is_same(&e2),
            (Expr::IfThenElse(c1, t1, l1, e1, _), Expr::IfThenElse(c2, t2, l2, e2, _)) => {
                c1.is_same(c2)
                    && t1.is_same(t2)
                    && l1.len() == l2.len()
                    && l1
                        .iter()
                        .zip(l2.iter())
                        .all(|((x1, y1), (x2, y2))| x1.is_same(x2) && y1.is_same(y2))
                    && e1.is_same(e2)
            }
            (Expr::Forall(p1, e1, _), Expr::Forall(p2, e2, _)) => {
                p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(x1, x2)| x1.is_same(x2))
                    && e1.is_same(e2)
            }
            (Expr::Exists(p1, e1, _), Expr::Exists(p2, e2, _)) => {
                p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(x1, x2)| x1.is_same(x2))
                    && e1.is_same(e2)
            }
            (Expr::Sum(p1, e1, _), Expr::Sum(p2, e2, _)) => {
                p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(x1, x2)| x1.is_same(x2))
                    && e1.is_same(e2)
            }
            (Expr::Prod(p1, e1, _), Expr::Prod(p2, e2, _)) => {
                p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(x1, x2)| x1.is_same(x2))
                    && e1.is_same(e2)
            }
            _ => false,
        }
    }

    pub fn all_same(v1: &Vec<Expr>, v2: &Vec<Expr>) -> bool {
        v1.iter().zip(v2.iter()).all(|(x, y)| x.is_same(y))
    }

    pub fn resolve_type(&self, entries: &TypeEntries) -> Result<Expr, Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(self.clone()),
            Expr::IntValue(_, _) => Ok(self.clone()),
            Expr::RealValue(_, _) => Ok(self.clone()),
            Expr::Prefix(o, e, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Prefix(*o, e, pos))
            }
            Expr::Binary(l, o, r, pos) => {
                let l = Box::new(l.resolve_type(entries)?);
                let r = Box::new(r.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Binary(l, *o, r, pos))
            }
            Expr::Variable(_, _) => Ok(self.clone()),
            Expr::Parameter(p) => {
                let mut p = p.clone();
                p.resolve_type(entries)?;
                Ok(Expr::Parameter(p))
            }
            Expr::FunctionCall(id, p, pos) => {
                let mut v = Vec::new();
                for x in p.iter() {
                    v.push(x.resolve_type(entries)?);
                }
                let pos = pos.clone();
                Ok(Expr::FunctionCall(*id, v, pos))
            }
            Expr::Instance(_, _) => Ok(self.clone()),
            Expr::StrucSelf(_, _) => Ok(self.clone()),
            Expr::StrucAttribute(e, id, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::StrucAttribute(e, *id, pos))
            }
            Expr::StrucMetCall(e, id, p, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let mut v = Vec::new();
                for x in p.iter() {
                    v.push(x.resolve_type(entries)?);
                }
                let pos = pos.clone();
                Ok(Expr::StrucMetCall(e, *id, v, pos))
            }
            Expr::ClassSelf(_, _) => Ok(self.clone()),
            Expr::ClassAttribute(e, id, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::ClassAttribute(e, *id, pos))
            }
            Expr::ClassMetCall(e, id, p, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let mut v = Vec::new();
                for x in p.iter() {
                    v.push(x.resolve_type(entries)?);
                }
                let pos = pos.clone();
                Ok(Expr::ClassMetCall(e, *id, v, pos))
            }
            Expr::AsClass(e, id) => {
                let e = Box::new(e.resolve_type(entries)?);
                Ok(Expr::AsClass(e, *id))
            }
            Expr::AsInterval(e, min, max, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::AsInterval(e, *min, *max, pos))
            }
            Expr::AsInt(e, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::AsInt(e, pos))
            }
            Expr::AsReal(e, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::AsReal(e, pos))
            }
            Expr::IfThenElse(c, t, l, e, pos) => {
                let c = Box::new(c.resolve_type(entries)?);
                let t = Box::new(t.resolve_type(entries)?);
                let mut v = Vec::new();
                for (x, y) in l.iter() {
                    let x = x.resolve_type(entries)?;
                    let y = y.resolve_type(entries)?;
                    v.push((x, y));
                }
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::IfThenElse(c, t, v, e, pos))
            }
            Expr::Forall(p, e, pos) => {
                let mut v = Vec::new();
                for x in p.iter() {
                    let mut x = x.clone();
                    x.resolve_type(entries)?;
                    v.push(x);
                }
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Forall(v, e, pos))
            }
            Expr::Exists(p, e, pos) => {
                let mut v = Vec::new();
                for x in p.iter() {
                    let mut x = x.clone();
                    x.resolve_type(entries)?;
                    v.push(x);
                }
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Exists(v, e, pos))
            }
            Expr::Sum(p, e, pos) => {
                let mut v = Vec::new();
                for x in p.iter() {
                    let mut x = x.clone();
                    x.resolve_type(entries)?;
                    v.push(x);
                }
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Sum(v, e, pos))
            }
            Expr::Prod(p, e, pos) => {
                let mut v = Vec::new();
                for x in p.iter() {
                    let mut x = x.clone();
                    x.resolve_type(entries)?;
                    v.push(x);
                }
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Prod(v, e, pos))
            }
            Expr::Unresolved(_, _) => Ok(self.clone()),
            Expr::UnresolvedFunCall(n, p, pos) => {
                let mut v = Vec::new();
                for x in p.iter() {
                    let x = x.resolve_type(entries)?;
                    v.push(x);
                }
                let pos = pos.clone();
                Ok(Expr::UnresolvedFunCall(n.into(), v, pos))
            }
            Expr::UnresolvedAttribute(e, n, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::UnresolvedAttribute(e, n.into(), pos))
            }
            Expr::UnresolvedMethCall(e, n, p, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let mut v = Vec::new();
                for x in p.iter() {
                    let x = x.resolve_type(entries)?;
                    v.push(x);
                }
                let pos = pos.clone();
                Ok(Expr::UnresolvedMethCall(e, n.into(), v, pos))
            }
        }
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
            Expr::Parameter(_) => Ok(self.clone()),
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
            Expr::AsClass(e, id) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::AsClass(Box::new(e), *id))
            }
            Expr::AsInterval(e, min, max, pos) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::AsInterval(Box::new(e), *min, *max, pos.clone()))
            }
            Expr::AsInt(e, pos) => {
                let e = Box::new(e.resolve(problem, entries)?);
                let pos = pos.clone();
                Ok(Expr::AsInt(e, pos))
            }
            Expr::AsReal(e, pos) => {
                let e = Box::new(e.resolve(problem, entries)?);
                let pos = pos.clone();
                Ok(Expr::AsReal(e, pos))
            }
            //
            Expr::IfThenElse(c, t, l, e, pos) => {
                let c = c.resolve(problem, entries)?;
                let t = t.resolve(problem, entries)?;
                let mut v = Vec::new();
                for (x, y) in l.iter() {
                    let x = x.resolve(problem, entries)?;
                    let y = y.resolve(problem, entries)?;
                    v.push((x, y));
                }
                let e = e.resolve(problem, entries)?;
                Ok(Expr::IfThenElse(
                    Box::new(c),
                    Box::new(t),
                    v,
                    Box::new(e),
                    pos.clone(),
                ))
            }
            Expr::Forall(p, e, pos) => {
                let mut entries = entries.clone();
                for x in p.iter() {
                    entries = entries.add(Entry::new_parameter(x));
                }
                //
                let p = p.clone();
                let e = Box::new(e.resolve(problem, &entries)?);
                let pos = pos.clone();
                Ok(Expr::Forall(p, e, pos))
            }
            Expr::Exists(p, e, pos) => {
                let mut entries = entries.clone();
                for x in p.iter() {
                    entries = entries.add(Entry::new_parameter(x));
                }
                //
                let p = p.clone();
                let e = Box::new(e.resolve(problem, &entries)?);
                let pos = pos.clone();
                Ok(Expr::Exists(p, e, pos))
            }
            Expr::Sum(p, e, pos) => {
                let mut entries = entries.clone();
                for x in p.iter() {
                    entries = entries.add(Entry::new_parameter(x));
                }
                //
                let p = p.clone();
                let e = Box::new(e.resolve(problem, &entries)?);
                let pos = pos.clone();
                Ok(Expr::Sum(p, e, pos))
            }
            Expr::Prod(p, e, pos) => {
                let mut entries = entries.clone();
                for x in p.iter() {
                    entries = entries.add(Entry::new_parameter(x));
                }
                //
                let p = p.clone();
                let e = Box::new(e.resolve(problem, &entries)?);
                let pos = pos.clone();
                Ok(Expr::Prod(p, e, pos))
            }
            //
            Expr::Unresolved(name, position) => match entries.get(&name) {
                Some(entry) => match entry.typ() {
                    EntryType::Variable(id) => Ok(Self::Variable(*id, position.clone())),
                    EntryType::Parameter(p) => Ok(Self::Parameter(p.clone())),
                    EntryType::Instance(id) => Ok(Expr::Instance(*id, position.clone())),
                    EntryType::StrucSelf(id) => Ok(Expr::StrucSelf(*id, position.clone())),
                    EntryType::ClassSelf(id) => Ok(Expr::ClassSelf(*id, position.clone())),
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
            Expr::Parameter(p) => p.typ().clone(),
            Expr::StrucSelf(id, _) => Type::Structure(*id),
            Expr::StrucAttribute(_, id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::StrucMetCall(_, id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::ClassSelf(id, _) => Type::Class(*id),
            Expr::ClassAttribute(_, id, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::ClassMetCall(_, id, _, _) => problem.get(*id).unwrap().typ().clone(),
            Expr::AsClass(_, id) => Type::Class(*id),
            Expr::AsInterval(_, min, max, _) => Type::Interval(*min, *max),
            Expr::AsInt(_, _) => Type::Int,
            Expr::AsReal(_, _) => Type::Real,
            Expr::IfThenElse(_, t, l, e, _) => {
                let mut res = t.typ(problem);
                for (_, x) in l.iter() {
                    res = res.common_type(problem, &x.typ(problem));
                }
                res = res.common_type(problem, &e.typ(problem));
                res
            }
            Expr::Forall(_, _, _) => Type::Bool,
            Expr::Exists(_, _, _) => Type::Bool,
            Expr::Sum(_, e, _) => match e.typ(problem) {
                t @ Type::Real => t,
                t @ Type::Int => t,
                Type::Interval(_, _) => Type::Int,
                _ => Type::Undefined,
            },
            Expr::Prod(_, e, _) => match e.typ(problem) {
                t @ Type::Real => t,
                t @ Type::Int => t,
                Type::Interval(_, _) => Type::Int,
                _ => Type::Undefined,
            },
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
            Expr::Parameter(_) => Ok(()),
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
            Expr::AsClass(e, id) => {
                check_subtype_type(problem, &Type::Class(*id), e, &e.typ(problem))
            }
            Expr::AsInterval(e, _, _, _) => check_type_integer(e, &e.typ(problem)),
            Expr::AsInt(e, _) => check_type_number(e, &e.typ(problem)),
            Expr::AsReal(e, _) => check_type_number(e, &e.typ(problem)),
            Expr::IfThenElse(c, t, l, e, _) => {
                // Bool
                check_type_bool(c, &c.typ(problem))?;
                for (x, _) in l {
                    check_type_bool(x, &x.typ(problem))?;
                }
                // Return Type
                let typ = &self.typ(problem);
                check_subtype_type(problem, typ, t, &t.typ(problem))?;
                for (_, y) in l {
                    check_subtype_type(problem, typ, y, &y.typ(problem))?;
                }
                check_subtype_type(problem, typ, e, &e.typ(problem))
            }
            Expr::Forall(_, e, _) => {
                e.check_type(problem)?;
                check_type_bool(e, &e.typ(problem))
            }
            Expr::Exists(_, e, _) => {
                e.check_type(problem)?;
                check_type_bool(e, &e.typ(problem))
            }
            Expr::Sum(_, e, _) => {
                e.check_type(problem)?;
                check_type_number(e, &e.typ(problem))
            }
            Expr::Prod(_, e, _) => {
                e.check_type(problem)?;
                check_type_number(e, &e.typ(problem))
            }
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
            Expr::UnresolvedAttribute(_, _, _) => panic!(),
            Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
        }
    }

    pub fn type_inference(&self, problem: &Problem) -> Expr {
        match self {
            Expr::BoolValue(_, _) => self.clone(),
            Expr::IntValue(_, _) => self.clone(),
            Expr::RealValue(_, _) => self.clone(),
            //
            Expr::Prefix(op, e, p) => {
                let e = e.type_inference(problem);
                Expr::Prefix(*op, Box::new(e), p.clone())
            }
            Expr::Binary(left, op, right, pos) => {
                let mut left = left.type_inference(problem);
                let mut right = right.type_inference(problem);
                let lt = &left.typ(problem);
                let rt = &right.typ(problem);
                match (lt, rt) {
                    (Type::Class(i1), Type::Class(i2)) => {
                        if i1 != i2 {
                            if lt.is_subtype_of(problem, rt) {
                                left = left.as_type(problem, rt);
                            } else {
                                right = right.as_type(problem, lt);
                            }
                        }
                    }
                    _ => {}
                }
                Expr::Binary(Box::new(left), *op, Box::new(right), pos.clone())
            }
            Expr::Variable(_, _) => self.clone(),
            Expr::Parameter(_) => self.clone(),
            Expr::FunctionCall(id, params, pos) => {
                let fun = problem.get(*id).unwrap();
                let mut v = Vec::new();
                for (e, t) in params.iter().zip(fun.parameters_type().iter()) {
                    let e = e.type_inference(problem);
                    let e = e.as_type(problem, t);
                    v.push(e);
                }
                Expr::FunctionCall(*id, v, pos.clone())
            }
            Expr::Instance(_, _) => self.clone(),
            Expr::StrucSelf(_, _) => self.clone(),
            Expr::StrucAttribute(e, id, pos) => {
                let e = e.type_inference(problem);
                Expr::StrucAttribute(Box::new(e), *id, pos.clone())
            }
            Expr::StrucMetCall(e, id, params, pos) => {
                let meth = problem.get(*id).unwrap();
                let e = e.type_inference(problem);
                let mut v = Vec::new();
                for (e, t) in params.iter().zip(meth.parameters_type().iter()) {
                    let e = e.type_inference(problem);
                    let e = e.as_type(problem, t);
                    v.push(e);
                }
                Expr::StrucMetCall(Box::new(e), *id, v, pos.clone())
            }
            Expr::ClassSelf(_, _) => self.clone(),
            Expr::ClassAttribute(e, id, pos) => {
                let e = e.type_inference(problem);
                let AttributeId(class_id, _) = id;
                let e = e.as_type(problem, &Type::Class(*class_id));
                Expr::ClassAttribute(Box::new(e), *id, pos.clone())
            }
            Expr::ClassMetCall(e, id, params, pos) => {
                let e = e.type_inference(problem);
                let MethodId(class_id, _) = id;
                let e = e.as_type(problem, &Type::Class(*class_id));
                //
                let meth = problem.get(*id).unwrap();
                let mut v = Vec::new();
                for (e, t) in params.iter().zip(meth.parameters_type().iter()) {
                    let e = e.type_inference(problem);
                    let e = e.as_type(problem, t);
                    v.push(e);
                }
                Expr::ClassMetCall(Box::new(e), *id, v, pos.clone())
            }
            Expr::AsClass(e, id) => {
                let e = e.type_inference(problem);
                Expr::AsClass(Box::new(e), *id)
            }
            Expr::AsInterval(e, min, max, pos) => {
                let e = e.type_inference(problem);
                Expr::AsInterval(Box::new(e), *min, *max, pos.clone())
            }
            Expr::AsInt(e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let pos = pos.clone();
                Expr::AsInt(e, pos)
            }
            Expr::AsReal(e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let pos = pos.clone();
                Expr::AsReal(e, pos)
            }
            Expr::IfThenElse(c, t, l, e, pos) => {
                let c = Box::new(c.type_inference(problem));
                let t = Box::new(t.type_inference(problem));
                let l = l
                    .iter()
                    .map(|(x, y)| (x.type_inference(problem), y.type_inference(problem)))
                    .collect();
                let e = Box::new(e.type_inference(problem));
                let pos = pos.clone();
                Expr::IfThenElse(c, t, l, e, pos)
            }
            Expr::Forall(p, e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let p = p.clone();
                let pos = pos.clone();
                Expr::Forall(p, e, pos)
            }
            Expr::Exists(p, e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let p = p.clone();
                let pos = pos.clone();
                Expr::Exists(p, e, pos)
            }
            Expr::Sum(p, e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let p = p.clone();
                let pos = pos.clone();
                Expr::Sum(p, e, pos)
            }
            Expr::Prod(p, e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let p = p.clone();
                let pos = pos.clone();
                Expr::Prod(p, e, pos)
            }
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
            Expr::UnresolvedAttribute(_, _, _) => panic!(),
            Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
        }
    }

    pub fn as_type(&self, problem: &Problem, expected: &Type) -> Expr {
        match (&self.typ(problem), expected) {
            (Type::Class(id1), Type::Class(id2)) => {
                if id1 == id2 {
                    self.clone()
                } else {
                    Expr::AsClass(Box::new(self.clone()), *id2)
                }
            }
            _ => self.clone(),
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
            Expr::Parameter(_) => Ok(()),
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
            Expr::AsClass(e, _) => e.check_parameter_size(problem),
            Expr::AsInterval(e, _, _, _) => e.check_parameter_size(problem),
            Expr::AsInt(e, _) => e.check_parameter_size(problem),
            Expr::AsReal(e, _) => e.check_parameter_size(problem),
            Expr::IfThenElse(c, t, l, e, _) => {
                c.check_parameter_size(problem)?;
                t.check_parameter_size(problem)?;
                for (x, y) in l.iter() {
                    x.check_parameter_size(problem)?;
                    y.check_parameter_size(problem)?;
                }
                e.check_parameter_size(problem)
            }
            Expr::Forall(_, e, _) => e.check_parameter_size(problem),
            Expr::Exists(_, e, _) => e.check_parameter_size(problem),
            Expr::Sum(_, e, _) => e.check_parameter_size(problem),
            Expr::Prod(_, e, _) => e.check_parameter_size(problem),
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
                Expr::Parameter(_) => self.clone(),
                Expr::StrucSelf(_, _) => self.clone(),
                Expr::StrucAttribute(e, id, pos) => {
                    Expr::StrucAttribute(Box::new(e.substitute(old, expr)), *id, pos.clone())
                }
                Expr::StrucMetCall(e, id, args, pos) => {
                    let e = e.substitute(old, expr);
                    let args = args.iter().map(|a| a.substitute(old, expr)).collect();
                    Expr::StrucMetCall(Box::new(e), *id, args, pos.clone())
                }
                Expr::ClassSelf(_, _) => self.clone(),
                Expr::ClassAttribute(e, id, pos) => {
                    Expr::ClassAttribute(Box::new(e.substitute(old, expr)), *id, pos.clone())
                }
                Expr::ClassMetCall(e, id, args, pos) => {
                    let e = e.substitute(old, expr);
                    let args = args.iter().map(|a| a.substitute(old, expr)).collect();
                    Expr::ClassMetCall(Box::new(e), *id, args, pos.clone())
                }
                Expr::AsClass(e, id) => Expr::AsClass(Box::new(e.substitute(old, expr)), *id),
                Expr::AsInterval(e, min, max, pos) => {
                    Expr::AsInterval(Box::new(e.substitute(old, expr)), *min, *max, pos.clone())
                }
                Expr::AsInt(e, pos) => {
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::AsInt(e, pos)
                }
                Expr::AsReal(e, pos) => {
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::AsReal(e, pos)
                }
                Expr::IfThenElse(c, t, l, e, pos) => {
                    let c = Box::new(c.substitute(old, expr));
                    let t = Box::new(t.substitute(old, expr));
                    let l = l
                        .iter()
                        .map(|(x, y)| (x.substitute(old, expr), y.substitute(old, expr)))
                        .collect();
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::IfThenElse(c, t, l, e, pos)
                }
                Expr::Forall(p, e, pos) => {
                    let p = p.clone();
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::Forall(p, e, pos)
                }
                Expr::Exists(p, e, pos) => {
                    let p = p.clone();
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::Exists(p, e, pos)
                }
                Expr::Sum(p, e, pos) => {
                    let p = p.clone();
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::Sum(p, e, pos)
                }
                Expr::Prod(p, e, pos) => {
                    let p = p.clone();
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::Prod(p, e, pos)
                }
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

pub fn check_type_integer(expr: &Expr, expr_type: &Type) -> Result<(), Error> {
    if expr_type.is_integer() {
        Ok(())
    } else {
        Err(Error::Type {
            expr: expr.clone(),
            typ: expr_type.clone(),
            expected: vec![Type::Int],
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
            Expr::Prefix(op, kid, _) => format!("({} {})", op, kid.to_lang(problem)),
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
            Expr::Parameter(p) => p.name().to_string(),
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
            Expr::AsClass(e, id) => format!(
                "({} as {})",
                e.to_lang(problem),
                problem.get(*id).unwrap().name()
            ),
            Expr::AsInterval(e, min, max, _) => {
                format!("({} as {}..{})", e.to_lang(problem), min, max)
            }
            Expr::AsInt(e, _) => format!("({} as Int)", e.to_lang(problem)),
            Expr::AsReal(e, _) => format!("({} as Real)", e.to_lang(problem)),
            Expr::IfThenElse(c, t, l, e, _) => {
                let mut s = format!("if {} then {}", c.to_lang(problem), t.to_lang(problem));
                for (x, y) in l.iter() {
                    s.push_str(&format!(
                        " elif {} then {}",
                        x.to_lang(problem),
                        y.to_lang(problem)
                    ));
                }
                s.push_str(&format!(" else {} end", e.to_lang(problem)));
                s
            }
            Expr::Forall(p, e, _) => {
                let mut s = "forall ".to_string();
                if let Some((first, others)) = p.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for x in others.iter() {
                        s.push_str(&format!(", {}", x.to_lang(problem)));
                    }
                }
                s.push_str(&format!(" | {} end", e.to_lang(problem)));
                s
            }
            Expr::Exists(p, e, _) => {
                let mut s = "exists ".to_string();
                if let Some((first, others)) = p.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for x in others.iter() {
                        s.push_str(&format!(", {}", x.to_lang(problem)));
                    }
                }
                s.push_str(&format!(" | {} end", e.to_lang(problem)));
                s
            }
            Expr::Sum(p, e, _) => {
                let mut s = "sum ".to_string();
                if let Some((first, others)) = p.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for x in others.iter() {
                        s.push_str(&format!(", {}", x.to_lang(problem)));
                    }
                }
                s.push_str(&format!(" | {} end", e.to_lang(problem)));
                s
            }
            Expr::Prod(p, e, _) => {
                let mut s = "prod ".to_string();
                if let Some((first, others)) = p.split_first() {
                    s.push_str(&first.to_lang(problem));
                    for x in others.iter() {
                        s.push_str(&format!(", {}", x.to_lang(problem)));
                    }
                }
                s.push_str(&format!(" | {} end", e.to_lang(problem)));
                s
            }
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
