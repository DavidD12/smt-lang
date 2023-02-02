use super::*;
use crate::parser::Position;
use crate::problem::WithPosition;

impl WithPosition for Expr {
    fn position(&self) -> &Option<Position> {
        match self {
            Expr::BoolValue(_, p) => p,
            Expr::IntValue(_, p) => p,
            Expr::RealValue(_, p) => p,
            Expr::Unary(_, _, p) => p,
            Expr::Binary(_, _, _, p) => p,
            Expr::Nary(_, _, p) => p,
            Expr::FunctionCall(_, _, p) => p,
            Expr::Variable(_, p) => p,
            Expr::Parameter(p) => &p.position(),
            Expr::Instance(_, p) => p,
            Expr::StrucSelf(_, p) => p,
            Expr::StrucAttribute(_, _, p) => p,
            Expr::StrucMetCall(_, _, _, p) => p,
            Expr::ClassSelf(_, p) => p,
            Expr::ClassAttribute(_, _, p) => p,
            Expr::ClassMetCall(_, _, _, p) => p,
            Expr::AsClass(_, _) => &None,
            Expr::AsInterval(_, _, _, p) => p,
            Expr::AsInt(_, p) => p,
            Expr::AsReal(_, p) => p,
            Expr::IfThenElse(_, _, _, _, p) => p,
            Expr::Quantifier(_, _, _, p) => p,
            Expr::Unresolved(_, p) => p,
            Expr::UnresolvedFunCall(_, _, p) => p,
            Expr::UnresolvedAttribute(_, _, p) => p,
            Expr::UnresolvedMethCall(_, _, _, p) => p,
        }
    }
}
