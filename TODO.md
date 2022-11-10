- homogeneiser les noms des expressions: Expr pas Expr
- rename typ() into type_()
- add methode dans Id to get superid
- minimize/maximize unbounded ? => NO !
- add check interval into expr with tth type conversion
- mutualiser les definition avec un trait with Expr et redefinition les check et resolve
- mutualiser les eq/ne/lt/.. commen dans Many
- remove &xxx.typ() dans smt et autres car .type() retourne une reference maintenant

- finir le refactor des parametres: with type & with ...