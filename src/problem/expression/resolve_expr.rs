use crate::problem::*;

impl Expr {
    pub fn resolve(&self, problem: &Problem, entries: &Entries) -> Result<Expr, Error> {
        match self {
            Expr::BoolValue(_, _) => Ok(self.clone()),
            Expr::IntValue(_, _) => Ok(self.clone()),
            Expr::RealValue(_, _) => Ok(self.clone()),
            //
            Expr::Unary(op, kid, position) => {
                let kid = kid.resolve(problem, entries)?;
                Ok(Self::Unary(*op, Box::new(kid), position.clone()))
            }
            Expr::Binary(left, op, right, position) => {
                let left = left.resolve(problem, entries)?;
                let right = right.resolve(problem, entries)?;
                Ok(Self::Binary(
                    Box::new(left),
                    *op,
                    Box::new(right),
                    position.clone(),
                ))
            }
            Expr::Nary(op, kids, position) => {
                let mut v: Vec<Expr> = vec![];
                for e in kids.iter() {
                    v.push(e.resolve(problem, entries)?);
                }
                Ok(Self::Nary(*op, v, position.clone()))
            }
            //
            Expr::FunctionCall(id, parameters, position) => {
                let mut v: Vec<Expr> = vec![];
                for p in parameters.iter() {
                    v.push(p.resolve(problem, entries)?);
                }
                Ok(Self::FunctionCall(*id, v, position.clone()))
            }
            //
            Expr::Variable(_, _) => Ok(self.clone()),
            Expr::Parameter(_) => Ok(self.clone()),
            Expr::Instance(_, _) => Ok(self.clone()),
            Expr::StrucSelf(_, _) => Ok(self.clone()),
            Expr::StrucAttribute(e, id, pos) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::StrucAttribute(Box::new(e), *id, pos.clone()))
            }
            Expr::StrucMetCall(e, id, args, pos) => {
                let e = e.resolve(problem, entries)?;
                let mut v: Vec<Expr> = vec![];
                for p in args.iter() {
                    v.push(p.resolve(problem, entries)?);
                }

                Ok(Expr::StrucMetCall(Box::new(e), *id, v, pos.clone()))
            }
            //
            Expr::ClassSelf(_, _) => Ok(self.clone()),
            Expr::ClassAttribute(e, id, pos) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::ClassAttribute(Box::new(e), *id, pos.clone()))
            }
            Expr::ClassMetCall(e, id, args, pos) => {
                let e = e.resolve(problem, entries)?;
                let mut v: Vec<Expr> = vec![];
                for p in args.iter() {
                    v.push(p.resolve(problem, entries)?);
                }

                Ok(Expr::ClassMetCall(Box::new(e), *id, v, pos.clone()))
            }
            Expr::AsClass(e, id) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::AsClass(Box::new(e), *id))
            }
            Expr::AsInterval(e, min, max, pos) => {
                let e = e.resolve(problem, entries)?;
                Ok(Expr::AsInterval(Box::new(e), *min, *max, pos.clone()))
            }
            Expr::AsInt(e, pos) => {
                let e = Box::new(e.resolve(problem, entries)?);
                let pos = pos.clone();
                Ok(Expr::AsInt(e, pos))
            }
            Expr::AsReal(e, pos) => {
                let e = Box::new(e.resolve(problem, entries)?);
                let pos = pos.clone();
                Ok(Expr::AsReal(e, pos))
            }
            //
            Expr::IfThenElse(c, t, l, e, pos) => {
                let c = c.resolve(problem, entries)?;
                let t = t.resolve(problem, entries)?;
                let mut v = Vec::new();
                for (x, y) in l.iter() {
                    let x = x.resolve(problem, entries)?;
                    let y = y.resolve(problem, entries)?;
                    v.push((x, y));
                }
                let e = e.resolve(problem, entries)?;
                Ok(Expr::IfThenElse(
                    Box::new(c),
                    Box::new(t),
                    v,
                    Box::new(e),
                    pos.clone(),
                ))
            }
            Expr::Quantifier(op, p, e, pos) => {
                let mut entries = entries.clone();
                for x in p.iter() {
                    entries = entries.add(Entry::new_parameter(x));
                }
                //
                let p = p.clone();
                let e = Box::new(e.resolve(problem, &entries)?);
                let pos = pos.clone();
                Ok(Expr::Quantifier(*op, p, e, pos))
            }
            //
            Expr::Unresolved(name, position) => match entries.get(&name) {
                Some(entry) => match entry.typ() {
                    EntryType::Variable(id) => Ok(Self::Variable(*id, position.clone())),
                    EntryType::Parameter(p) => Ok(Self::Parameter(p.clone())),
                    EntryType::Instance(id) => Ok(Expr::Instance(*id, position.clone())),
                    EntryType::StrucSelf(id) => Ok(Expr::StrucSelf(*id, position.clone())),
                    EntryType::ClassSelf(id) => Ok(Expr::ClassSelf(*id, position.clone())),
                },
                None => Err(Error::Resolve {
                    category: "identifier".to_string(),
                    name: name.clone(),
                    position: position.clone(),
                }),
            },
            Expr::UnresolvedFunCall(name, params, position) => {
                let mut v: Vec<Expr> = vec![];
                for p in params.iter() {
                    v.push(p.resolve(problem, entries)?);
                }
                if let Some(function) = problem.find_function(name) {
                    return Ok(Expr::FunctionCall(function.id(), v, position.clone()));
                }
                Err(Error::Resolve {
                    category: "function".to_string(),
                    name: name.clone(),
                    position: position.clone(),
                })
            }
            Expr::UnresolvedAttribute(e, name, pos) => {
                let e = e.resolve(problem, entries)?;
                let t = e.typ(problem);
                println!();
                println!(">>>>> e: {} <<<<<", e.to_lang(problem));
                println!(">>>>> t: {} <<<<<", t.to_lang(problem));
                println!(">>>>> entries: {:#?} <<<<<", entries);
                if let Type::Structure(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_attribute(name) {
                        Ok(Expr::StrucAttribute(Box::new(e), a.id(), pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "attibute for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else if let Type::Class(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_all_attribute(problem, name) {
                        Ok(Expr::ClassAttribute(Box::new(e), a.id(), pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "attibute for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else {
                    panic!("unresolved expr {}", self.to_lang(problem))
                }
            }
            Expr::UnresolvedMethCall(e, name, args, pos) => {
                let e = e.resolve(problem, entries)?;
                let mut v: Vec<Expr> = vec![];
                for p in args.iter() {
                    v.push(p.resolve(problem, entries)?);
                }
                let t = e.typ(problem);
                if let Type::Structure(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_method(name) {
                        Ok(Expr::StrucMetCall(Box::new(e), a.id(), v, pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "method for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else if let Type::Class(id) = t {
                    if let Some(a) = problem.get(id).unwrap().find_all_method(problem, name) {
                        Ok(Expr::ClassMetCall(Box::new(e), a.id(), v, pos.clone()))
                    } else {
                        Err(Error::Resolve {
                            category: format!(
                                "method for type '{}'",
                                e.typ(problem).to_lang(problem)
                            ),
                            name: name.clone(),
                            position: pos.clone(),
                        })
                    }
                } else {
                    panic!()
                }
            }
        }
    }
}
