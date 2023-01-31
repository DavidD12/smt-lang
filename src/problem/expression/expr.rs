use crate::parser::Position;
use fraction::Fraction;

use crate::problem::*;

//-------------------------------------------------- Unary --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnaryOp {
    Not,
    Minus,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not => write!(f, "not"),
            Self::Minus => write!(f, "-"),
        }
    }
}

//-------------------------------------------------- Bin --------------------------------------------------

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
            Self::Ne => write!(f, "!="),
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

//-------------------------------------------------- Qt --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum QtOp {
    Forall,
    Exists,
    Sum,
    Prod,
}

impl std::fmt::Display for QtOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QtOp::Forall => write!(f, "forall"),
            QtOp::Exists => write!(f, "exists"),
            QtOp::Sum => write!(f, "sum"),
            QtOp::Prod => write!(f, "prod"),
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
    Unary(UnaryOp, Box<Expr>, Option<Position>),
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
    Quantifier(QtOp, Vec<Parameter>, Box<Expr>, Option<Position>),
    //
    Unresolved(String, Option<Position>),
    UnresolvedFunCall(String, Vec<Expr>, Option<Position>),
    UnresolvedAttribute(Box<Expr>, String, Option<Position>),
    UnresolvedMethCall(Box<Expr>, String, Vec<Expr>, Option<Position>),
}

//------------------------- To Lang -------------------------

impl ToLang for Expr {
    fn to_lang(&self, problem: &Problem) -> String {
        match self {
            Expr::BoolValue(value, _) => format!("{}", value),
            Expr::IntValue(value, _) => format!("{}", value),
            Expr::RealValue(value, _) => format!("{}", value),
            Expr::Unary(op, kid, _) => format!("({} {})", op, kid.to_lang(problem)),
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
            Expr::Quantifier(op, p, e, _) => {
                let mut s = format!("{} ", op);
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
