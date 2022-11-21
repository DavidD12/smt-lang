- rename typ() into type_()
- mutualiser les eq/ne/lt/.. commen dans Many

- add solver timeout
- print something while computing before solution

- check bounded forall
- check expression interval
- alldiff
- Count
- change is empty to be less restrictive !
- Tuples
- Array
- precalcul ?
- instance (struct and class) specification:
  inst a1, a2: A {
    i = 10
    a = self.a
  }

- refactor all expression code: expression container: expression + position + type. resolve type change type

- multi files: define all before, and then resolve all