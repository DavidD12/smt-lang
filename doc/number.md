## Number Expressions

Numbers can be either unbounded integer (**Int**), integer interval (**min..max**), or real (**Real**). All those types have the following operations: **+** (add), **-** (substract or prefix negate), and **\*** (multiply). For those operations, the result type keeps its "kind"...

- Example:
```
let i, j: Int
let x, y: Real

constraint c_int = (i + 10 - j = i * 10)
constraint c_real = (x + 10.0 - y = x * 10.0)
```

- Solution:
```
let i: Int = 0
let j: Int = 10
let x: Real = 10/9
let y: Real = 0
```

### Interval

The interval type checker is a bit more complex. Each constant value "v" has an interval type *v..v*, and thus "v + 1" expression has type *(v+1)..(v+1)* and so on for each operation.
Moreover, the integer interval type is a subtype of the unbounded integer type.

- Example:
```
let i: 0..9
let j: 2..20
let k: Int = i + j

constraint c1 = (j = (i+1) * 2)
```

- Solution:
```
let i: 0..9 = 0
let j: 2..20 = 2
let k: Int = 2
```

### Division

An integer divided by another integer is a fraction and thus a Real.

- Example:
```
let i, j: Int
let x: Real = i / j

constraint c = (i > 0 and j >= 5 * i)
```

- Solution:
```
let i: Int = 1
let j: Int = 5
let x: Real = 1/5
```


