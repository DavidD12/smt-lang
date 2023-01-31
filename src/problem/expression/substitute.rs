use crate::problem::*;

impl Expr {
    pub fn substitute(&self, old: &Expr, expr: &Expr) -> Expr {
        if self.is_same(old) {
            expr.clone()
        } else {
            match self {
                Expr::BoolValue(_, _) => self.clone(),
                Expr::IntValue(_, _) => self.clone(),
                Expr::RealValue(_, _) => self.clone(),
                Expr::Unary(op, e, pos) => {
                    Expr::Unary(*op, Box::new(e.substitute(old, expr)), pos.clone())
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
                Expr::Quantifier(op, p, e, pos) => {
                    let p = p.clone();
                    let e = Box::new(e.substitute(old, expr));
                    let pos = pos.clone();
                    Expr::Quantifier(*op, p, e, pos)
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
