use super::*;

pub trait IsSame {
    fn is_same(&self, other: &Self) -> bool;

    fn all_same(v1: &Vec<Expr>, v2: &Vec<Expr>) -> bool {
        v1.iter().zip(v2.iter()).all(|(x, y)| x.is_same(y))
    }
}

impl IsSame for Expr {
    fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::BoolValue(x, _), Expr::BoolValue(y, _)) => x == y,
            (Expr::IntValue(x, _), Expr::IntValue(y, _)) => x == y,
            (Expr::RealValue(x, _), Expr::RealValue(y, _)) => x == y,
            (Expr::Unary(o1, e1, _), Expr::Unary(o2, e2, _)) => o1 == o2 && e1.is_same(e2),
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
            (Expr::Quantifier(op1, p1, e1, _), Expr::Quantifier(op2, p2, e2, _)) => {
                op1 == op2
                    && p1.len() == p2.len()
                    && p1.iter().zip(p2.iter()).all(|(x1, x2)| x1.is_same(x2))
                    && e1.is_same(e2)
            }
            _ => false,
        }
    }
}
