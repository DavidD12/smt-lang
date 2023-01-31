use crate::problem::*;

impl Expr {
    pub fn check_type(&self, problem: &Problem) -> Result<(), Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(()),
            Expr::IntValue(_, _) => Ok(()),
            Expr::RealValue(_, _) => Ok(()),
            Expr::Unary(op, e, _) => {
                e.check_type(problem)?;
                let e_type = e.typ(problem);
                match op {
                    UnaryOp::Not => check_type_bool(e, &e_type),
                    UnaryOp::Minus => check_type_number(e, &e_type),
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
            Expr::Quantifier(op, _, e, _) => {
                e.check_type(problem)?;
                match op {
                    QtOp::Forall => check_type_bool(e, &e.typ(problem)),
                    QtOp::Exists => check_type_bool(e, &e.typ(problem)),
                    QtOp::Sum => check_type_number(e, &e.typ(problem)),
                    QtOp::Prod => check_type_number(e, &e.typ(problem)),
                }
            }
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
            Expr::UnresolvedAttribute(_, _, _) => panic!(),
            Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
        }
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
