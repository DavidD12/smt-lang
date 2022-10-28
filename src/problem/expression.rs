use super::*;
use crate::error::Error;
use crate::parser::Position;
use fraction::Fraction;

use super::{Named, ToLang, VariableId};

//-------------------------------------------------- Bin Operator --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone)]
pub enum Expr {
    BoolValue(bool, Option<Position>),
    IntValue(isize, Option<Position>),
    RealValue(Fraction, Option<Position>),
    //
    PreExpr(PreOp, Box<Expr>, Option<Position>),
    //
    BinExpr(Box<Expr>, BinOp, Box<Expr>, Option<Position>),
    //
    Variable(VariableId, Option<Position>),
    //
    Unresolved(String, Option<Position>),
}

impl Expr {
    pub fn position(&self) -> &Option<Position> {
        match self {
            Expr::BoolValue(_, p) => p,
            Expr::IntValue(_, p) => p,
            Expr::RealValue(_, p) => p,
            Expr::PreExpr(_, _, p) => p,
            Expr::BinExpr(_, _, _, p) => p,
            Expr::Variable(_, p) => p,
            Expr::Unresolved(_, p) => p,
        }
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
            Expr::Variable(_, _) => Ok(self.clone()),
            //
            Expr::Unresolved(name, position) => match entries.get(&name) {
                Some(entry) => match entry.typ() {
                    EntryType::Variable(id) => Ok(Self::Variable(id, position.clone())),
                    EntryType::Function(_) => todo!(),
                },
                None => Err(Error::Resolve {
                    name: name.clone(),
                    position: position.clone(),
                }),
            },
        }
    }

    pub fn typ(&self, problem: &Problem) -> Type {
        match self {
            Expr::BoolValue(_, _) => Type::Bool,
            Expr::IntValue(_, _) => Type::Int,
            Expr::RealValue(_, _) => Type::Real,
            Expr::PreExpr(op, e, _) => match op {
                PreOp::Not => Type::Bool,
                PreOp::Minus => match e.typ(problem) {
                    Type::Int => Type::Int,
                    Type::Real => Type::Real,
                    _ => Type::Undefined,
                },
            },
            Expr::BinExpr(left, op, _, _) => match op {
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
                BinOp::Add => match left.typ(problem) {
                    Type::Int => Type::Int,
                    Type::Real => Type::Real,
                    _ => Type::Undefined,
                },
                BinOp::Sub => match left.typ(problem) {
                    Type::Int => Type::Int,
                    Type::Real => Type::Real,
                    _ => Type::Undefined,
                },
                BinOp::Mul => match left.typ(problem) {
                    Type::Int => Type::Int,
                    Type::Real => Type::Real,
                    _ => Type::Undefined,
                },
                BinOp::Div => Type::Real,
            },
            Expr::Variable(id, _) => problem.get(*id).unwrap().typ(),
            Expr::Unresolved(_, _) => Type::Undefined,
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
                    PreOp::Not => check_type(e, e_type, &[Type::Bool]),
                    PreOp::Minus => check_type(e, e_type, &[Type::Int, Type::Real]),
                }
            }
            Expr::BinExpr(l, op, r, _) => {
                l.check_type(problem)?;
                r.check_type(problem)?;
                let l_type = l.typ(problem);
                let r_type = r.typ(problem);
                // Eq/Ne
                if [BinOp::Eq, BinOp::Ne].contains(op) {
                    check_same_type(l_type, r, r_type)
                }
                // Compare
                else if [BinOp::Lt, BinOp::Le, BinOp::Ge, BinOp::Gt].contains(op) {
                    check_type(l, l_type, &[Type::Int, Type::Real])?;
                    check_same_type(l_type, r, r_type)
                }
                // Bool
                else if [BinOp::And, BinOp::Or, BinOp::Implies].contains(op) {
                    check_type(l, l_type, &[Type::Bool])?;
                    check_type(r, r_type, &[Type::Bool])
                }
                // Arith
                else if [BinOp::Add, BinOp::Sub, BinOp::Mul].contains(op) {
                    check_type(l, l_type, &[Type::Int, Type::Real])?;
                    check_same_type(l_type, r, r_type)
                }
                // Div
                else if *op == BinOp::Div {
                    check_type(l, l_type, &[Type::Int, Type::Real])?;
                    check_type(r, r_type, &[Type::Int, Type::Real])
                } else {
                    panic!("undefined")
                }
            }
            Expr::Variable(_, _) => Ok(()),
            Expr::Unresolved(_, _) => check_type(self, self.typ(problem), &[]),
        }
    }
}

pub fn check_type(expr: &Expr, expr_type: Type, expected: &[Type]) -> Result<(), Error> {
    if expected.contains(&expr_type) {
        Ok(())
    } else {
        Err(Error::Type {
            expr: expr.clone(),
            typ: expr_type,
            expected: expected.to_vec(),
        })
    }
}

fn check_same_type(left_type: Type, right: &Expr, right_type: Type) -> Result<(), Error> {
    if left_type == right_type {
        Ok(())
    } else {
        Err(Error::Type {
            expr: right.clone(),
            typ: right_type,
            expected: vec![left_type],
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
            Expr::PreExpr(op, kid, _) => format!("({} {}", op, kid.to_lang(problem)),
            Expr::BinExpr(left, op, right, _) => format!(
                "({} {} {})",
                left.to_lang(problem),
                op,
                right.to_lang(problem)
            ),
            Expr::Variable(id, _) => problem.get_variable(*id).unwrap().name().into(),
            Expr::Unresolved(name, _) => format!("{}?", name),
        }
    }
}
