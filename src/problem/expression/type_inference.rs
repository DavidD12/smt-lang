use crate::problem::*;

impl Expr {
    pub fn type_inference(&self, problem: &Problem) -> Expr {
        match self {
            Expr::BoolValue(_, _) => self.clone(),
            Expr::IntValue(_, _) => self.clone(),
            Expr::RealValue(_, _) => self.clone(),
            //
            Expr::Unary(op, e, p) => {
                let e = e.type_inference(problem);
                Expr::Unary(*op, Box::new(e), p.clone())
            }
            Expr::Binary(left, op, right, pos) => {
                let mut left = left.type_inference(problem);
                let mut right = right.type_inference(problem);
                let lt = &left.typ(problem);
                let rt = &right.typ(problem);
                match (lt, rt) {
                    (Type::Class(i1), Type::Class(i2)) => {
                        if i1 != i2 {
                            if lt.is_subtype_of(problem, rt) {
                                left = left.as_type(problem, rt);
                            } else {
                                right = right.as_type(problem, lt);
                            }
                        }
                    }
                    _ => {}
                }
                Expr::Binary(Box::new(left), *op, Box::new(right), pos.clone())
            }
            Expr::Nary(o, v, p) => {
                let l = v.iter().map(|e| e.type_inference(problem)).collect();
                Expr::Nary(*o, l, p.clone())
            }
            Expr::Variable(_, _) => self.clone(),
            Expr::Parameter(_) => self.clone(),
            Expr::FunctionCall(id, params, pos) => {
                let fun = problem.get(*id).unwrap();
                let mut v = Vec::new();
                for (e, t) in params.iter().zip(fun.parameters_type().iter()) {
                    let e = e.type_inference(problem);
                    let e = e.as_type(problem, t);
                    v.push(e);
                }
                Expr::FunctionCall(*id, v, pos.clone())
            }
            Expr::Instance(_, _) => self.clone(),
            Expr::StrucSelf(_, _) => self.clone(),
            Expr::StrucAttribute(e, id, pos) => {
                let e = e.type_inference(problem);
                Expr::StrucAttribute(Box::new(e), *id, pos.clone())
            }
            Expr::StrucMetCall(e, id, params, pos) => {
                let meth = problem.get(*id).unwrap();
                let e = e.type_inference(problem);
                let mut v = Vec::new();
                for (e, t) in params.iter().zip(meth.parameters_type().iter()) {
                    let e = e.type_inference(problem);
                    let e = e.as_type(problem, t);
                    v.push(e);
                }
                Expr::StrucMetCall(Box::new(e), *id, v, pos.clone())
            }
            Expr::ClassSelf(_, _) => self.clone(),
            Expr::ClassAttribute(e, id, pos) => {
                let e = e.type_inference(problem);
                let AttributeId(class_id, _) = id;
                let e = e.as_type(problem, &Type::Class(*class_id));
                Expr::ClassAttribute(Box::new(e), *id, pos.clone())
            }
            Expr::ClassMetCall(e, id, params, pos) => {
                let e = e.type_inference(problem);
                let MethodId(class_id, _) = id;
                let e = e.as_type(problem, &Type::Class(*class_id));
                //
                let meth = problem.get(*id).unwrap();
                let mut v = Vec::new();
                for (e, t) in params.iter().zip(meth.parameters_type().iter()) {
                    let e = e.type_inference(problem);
                    let e = e.as_type(problem, t);
                    v.push(e);
                }
                Expr::ClassMetCall(Box::new(e), *id, v, pos.clone())
            }
            Expr::AsClass(e, id) => {
                let e = e.type_inference(problem);
                Expr::AsClass(Box::new(e), *id)
            }
            Expr::AsInterval(e, min, max, pos) => {
                let e = e.type_inference(problem);
                Expr::AsInterval(Box::new(e), *min, *max, pos.clone())
            }
            Expr::AsInt(e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let pos = pos.clone();
                Expr::AsInt(e, pos)
            }
            Expr::AsReal(e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let pos = pos.clone();
                Expr::AsReal(e, pos)
            }
            Expr::IfThenElse(c, t, l, e, pos) => {
                let c = Box::new(c.type_inference(problem));
                let t = Box::new(t.type_inference(problem));
                let l = l
                    .iter()
                    .map(|(x, y)| (x.type_inference(problem), y.type_inference(problem)))
                    .collect();
                let e = Box::new(e.type_inference(problem));
                let pos = pos.clone();
                Expr::IfThenElse(c, t, l, e, pos)
            }
            Expr::Quantifier(op, p, e, pos) => {
                let e = Box::new(e.type_inference(problem));
                let p = p.clone();
                let pos = pos.clone();
                Expr::Quantifier(*op, p, e, pos)
            }
            Expr::Unresolved(_, _) => panic!(),
            Expr::UnresolvedFunCall(_, _, _) => panic!(),
            Expr::UnresolvedAttribute(_, _, _) => panic!(),
            Expr::UnresolvedMethCall(_, _, _, _) => panic!(),
        }
    }

    pub fn as_type(&self, problem: &Problem, expected: &Type) -> Expr {
        match (&self.typ(problem), expected) {
            (Type::Class(id1), Type::Class(id2)) => {
                if id1 == id2 {
                    self.clone()
                } else {
                    Expr::AsClass(Box::new(self.clone()), *id2)
                }
            }
            _ => self.clone(),
        }
    }
}
