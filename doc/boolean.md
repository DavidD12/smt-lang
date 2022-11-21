## Boolean Expression

The classical boolean operators are: **not**(prefix negate), **and**(conjunction), **or**(disjunction) and **=>** (Implies). The boolean values are: **true** and **false**. Notice that the constraints must be boolean expressions.


```
let x: Bool = true
let y: Bool = false
let z: Bool
constraint c = (x or not z and true => true)
```

If the verbose option is set to 2 (at least), we can see that the boolean expression priority behave as expected:
```
smt-lang -f boolean.sl -v2
```

```
...
constraint c = ((x or ((not z) and true)) => true)
...
```

