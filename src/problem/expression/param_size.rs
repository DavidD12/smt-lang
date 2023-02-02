use crate::problem::*;

impl Expr {
    pub fn check_parameter_size(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(()),
            Expr::IntValue(_, _) => Ok(()),
            Expr::RealValue(_, _) => Ok(()),
            Expr::Unary(_, e, _) => e.check_type(problem),
            Expr::Binary(l, _, r, _) => {
                l.check_parameter_size(problem)?;
                r.check_parameter_size(problem)
            }
            Expr::Nary(_, v, _) => {
                for e in v.iter() {
                    e.check_parameter_size(problem)?;
                }
                Ok(())
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
            Expr::Quantifier(_, _, e, _) => e.check_parameter_size(problem),
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
            Expr::UnresolvedAttribute(_, _, _) => panic!(),
            Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
        }
    }
}
