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
    PreExpr(PreOp, Box<Expr>, Option<Position>),
    //
    BinExpr(Box<Expr>, BinOp, Box<Expr>, Option<Position>),
    //
    FunctionCall(FunctionId, Vec<Expr>, Option<Position>),
    //
    Variable(VariableId, Option<Position>),
    Parameter(ParameterId, Option<Position>),
    //
    Unresolved(String, Option<Position>),
    UnresolvedFunCall(String, Vec<Expr>, Option<Position>),
}

impl Expr {
    pub fn position(&self) -> &Option<Position> {
        match self {
            Expr::BoolValue(_, p) => p,
            Expr::IntValue(_, p) => p,
            Expr::RealValue(_, p) => p,
            Expr::PreExpr(_, _, p) => p,
            Expr::BinExpr(_, _, _, p) => p,
            Expr::FunctionCall(_, _, p) => p,
            Expr::Variable(_, p) => p,
            Expr::Parameter(_, p) => p,
            Expr::Unresolved(_, p) => p,
            Expr::UnresolvedFunCall(_, _, p) => p,
        }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::BoolValue(x, _), Expr::BoolValue(y, _)) => x == y,
            (Expr::IntValue(x, _), Expr::IntValue(y, _)) => x == y,
            (Expr::RealValue(x, _), Expr::RealValue(y, _)) => x == y,
            (Expr::PreExpr(o1, e1, _), Expr::PreExpr(o2, e2, _)) => o1 == o2 && e1.is_same(e2),
            (Expr::BinExpr(l1, o1, r1, _), Expr::BinExpr(l2, o2, r2, _)) => {
                o1 == o2 && l1.is_same(l2) && r1.is_same(r2)
            }
            (Expr::FunctionCall(i1, p1, _), Expr::FunctionCall(i2, p2, _)) => {
                i1 == i2 && Self::all_same(p1, p2)
            }
            (Expr::Variable(i1, _), Expr::Variable(i2, _)) => i1 == i2,
            (Expr::Parameter(i1, _), Expr::Parameter(i2, _)) => i1 == i2,
            _ => false,
        }
    }

    pub fn all_same(v1: &Vec<Expr>, v2: &Vec<Expr>) -> bool {
        v1.iter().zip(v2.iter()).all(|(x, y)| x.is_same(y))
    }

    pub fn resolve(&self, entries: &Entries) -> Result<Expr, Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(self.clone()),
            Expr::IntValue(_, _) => Ok(self.clone()),
            Expr::RealValue(_, _) => Ok(self.clone()),
            //
            Expr::PreExpr(op, kid, position) => {
                let kid = kid.resolve(entries)?;
                Ok(Self::PreExpr(*op, Box::new(kid), position.clone()))
            }
            Expr::BinExpr(left, op, right, position) => {
                let left = left.resolve(entries)?;
                let right = right.resolve(entries)?;
                Ok(Self::BinExpr(
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
                    v.push(p.resolve(entries)?);
                }
                Ok(Self::FunctionCall(*id, v, position.clone()))
            }
            //
            Expr::Variable(_, _) => Ok(self.clone()),
            Expr::Parameter(_, _) => Ok(self.clone()),
            //
            Expr::Unresolved(name, position) => match entries.get(&name) {
                Some(entry) => match entry.typ() {
                    EntryType::Variable(id) => Ok(Self::Variable(id, position.clone())),
                    EntryType::Parameter(id) => Ok(Self::Parameter(id, position.clone())),
                    _ => Err(Error::Resolve {
                        name: name.clone(),
                        position: position.clone(),
                    }),
                },
                None => Err(Error::Resolve {
                    name: name.clone(),
                    position: position.clone(),
                }),
            },
            Expr::UnresolvedFunCall(name, params, position) => {
                let mut v: Vec<Expr> = vec![];
                for p in params.iter() {
                    v.push(p.resolve(entries)?);
                }
                if let Some(entry) = entries.get(&name) {
                    if let EntryType::Function(id) = entry.typ() {
                        return Ok(Expr::FunctionCall(id, v, position.clone()));
                    }
                }
                Err(Error::Resolve {
                    name: name.clone(),
                    position: position.clone(),
                })
            }
        }
    }

    pub fn typ(&self, problem: &Problem) -> Type {
        match self {
            Expr::BoolValue(_, _) => Type::Bool,
            Expr::IntValue(value, _) => Type::Interval(*value, *value),
            Expr::RealValue(_, _) => Type::Real,
            Expr::PreExpr(op, e, _) => match op {
                PreOp::Not => Type::Bool,
                PreOp::Minus => match e.typ(problem) {
                    Type::Int => Type::Int,
                    Type::Real => Type::Real,
                    Type::Interval(min, max) => Type::Interval(-max, -min),
                    _ => Type::Undefined,
                },
            },
            Expr::BinExpr(left, op, right, _) => match op {
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
            Expr::FunctionCall(id, _, _) => problem.get(*id).unwrap().return_type(),
            Expr::Variable(id, _) => problem.get(*id).unwrap().typ(),
            Expr::Parameter(id, _) => problem.get(*id).unwrap().typ(),
            Expr::Unresolved(_, _) => Type::Undefined,
            Expr::UnresolvedFunCall(_, _, _) => Type::Undefined,
        }
    }

    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(()),
            Expr::IntValue(_, _) => Ok(()),
            Expr::RealValue(_, _) => Ok(()),
            Expr::PreExpr(op, e, _) => {
                e.check_type(problem)?;
                let e_type = e.typ(problem);
                match op {
                    PreOp::Not => check_type_bool(e, &e_type),
                    PreOp::Minus => check_type_number(e, &e_type),
                }
            }
            Expr::BinExpr(l, op, r, _) => {
                l.check_type(problem)?;
                r.check_type(problem)?;
                let l_type = l.typ(problem);
                let r_type = r.typ(problem);
                // Eq/Ne
                if [BinOp::Eq, BinOp::Ne].contains(op) {
                    check_compatible_type(&l_type, r, &r_type)
                }
                // Compare
                else if [BinOp::Lt, BinOp::Le, BinOp::Ge, BinOp::Gt].contains(op) {
                    check_type_number(l, &l_type)?;
                    check_type_number(r, &r_type)?;
                    check_compatible_type(&l_type, r, &r_type)
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
                    check_compatible_type(&l_type, r, &r_type)
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
                    check_subtype_type(&p.typ(), e, &e.typ(problem))?;
                }
                Ok(())
            }
            Expr::Variable(_, _) => Ok(()),
            Expr::Parameter(_, _) => Ok(()),
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
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
                Expr::PreExpr(op, e, pos) => {
                    Expr::PreExpr(*op, Box::new(e.substitute(old, expr)), pos.clone())
                }
                Expr::BinExpr(left, op, right, pos) => {
                    let left = Box::new(left.substitute(old, expr));
                    let right = Box::new(right.substitute(old, expr));
                    Expr::BinExpr(left, *op, right, pos.clone())
                }
                Expr::FunctionCall(id, params, pos) => {
                    let params = params.iter().map(|p| p.substitute(old, expr)).collect();
                    Expr::FunctionCall(*id, params, pos.clone())
                }
                Expr::Variable(_, _) => self.clone(),
                Expr::Parameter(_, _) => self.clone(),
                Expr::Unresolved(_, _) => self.clone(),
                Expr::UnresolvedFunCall(_, _, _) => panic!(),
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
    left_type: &Type,
    right: &Expr,
    right_type: &Type,
) -> Result<(), Error> {
    if right_type.is_compatible_with(left_type) {
        Ok(())
    } else {
        Err(Error::Type {
            expr: right.clone(),
            typ: right_type.clone(),
            expected: vec![left_type.clone()],
        })
    }
}

pub fn check_subtype_type(left_type: &Type, right: &Expr, right_type: &Type) -> Result<(), Error> {
    if right_type.is_subtype_of(left_type) {
        Ok(())
    } else {
        Err(Error::Type {
            expr: right.clone(),
            typ: right_type.clone(),
            expected: vec![left_type.clone()],
        })
    }
}

// pub fn check_eq_type(left_type: &Type, right: &Expr, right_type: &Type) -> Result<(), Error> {
//     if right_type == left_type {
//         Ok(())
//     } else {
//         Err(Error::Type {
//             expr: right.clone(),
//             typ: right_type.clone(),
//             expected: vec![left_type.clone()],
//         })
//     }
// }

//------------------------- To Lang -------------------------

impl ToLang for Expr {
    fn to_lang(&self, problem: &super::Problem) -> String {
        match self {
            Expr::BoolValue(value, _) => format!("{}", value),
            Expr::IntValue(value, _) => format!("{}", value),
            Expr::RealValue(value, _) => format!("{}", value),
            Expr::PreExpr(op, kid, _) => format!("({} {}", op, kid.to_lang(problem)),
            Expr::BinExpr(left, op, right, _) => format!(
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
            Expr::Variable(id, _) => problem.get(*id).unwrap().name().into(),
            Expr::Parameter(id, _) => problem.get(*id).unwrap().name().into(),
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
        }
    }
}
