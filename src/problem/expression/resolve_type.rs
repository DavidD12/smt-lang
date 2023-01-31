use super::*;
use crate::problem::*;

impl Expr {
    pub fn resolve_type(&self, entries: &TypeEntries) -> Result<Expr, Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(self.clone()),
            Expr::IntValue(_, _) => Ok(self.clone()),
            Expr::RealValue(_, _) => Ok(self.clone()),
            Expr::Unary(o, e, pos) => {
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Unary(*o, e, pos))
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
            Expr::Quantifier(op, p, e, pos) => {
                let mut v = Vec::new();
                for x in p.iter() {
                    let mut x = x.clone();
                    x.resolve_type(entries)?;
                    v.push(x);
                }
                let e = Box::new(e.resolve_type(entries)?);
                let pos = pos.clone();
                Ok(Expr::Quantifier(*op, v, e, pos))
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
}
