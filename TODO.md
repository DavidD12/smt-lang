- rename typ() into type_()
- mutualiser les eq/ne/lt/.. commen dans Many
- remove &xxx.typ() dans smt et autres car .type() retourne une reference maintenant

- add solver timeout
- print something while computing before solution

- check bounded forall
- check expression interval
- alldiff
- Count
- change is empty to be less restrictive !

- refactor all expression code: expression container: expression + position + type. resolve type change type