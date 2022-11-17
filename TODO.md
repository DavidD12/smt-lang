- rename typ() into type_()
- mutualiser les definition avec un trait with Expr et redefinition les check et resolve
- mutualiser les eq/ne/lt/.. commen dans Many
- remove &xxx.typ() dans smt et autres car .type() retourne une reference maintenant

- add solver timeout
- print something while computing before solution

- check non cyclic extends
- check bounded forall
- check expression interval
- alldiff
- Count

- refactor all expression code: expression container: expression + position + type. resolve type change type