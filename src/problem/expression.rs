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
