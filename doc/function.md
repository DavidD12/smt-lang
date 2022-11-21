## Function

Function can be defined with the **let** keyword. The parameters type of the functions must be bounded. The real and unbounded integer types cannot be used on paramters. The result type as no limitation.

- Example:
```
let fun(i: 1..3, b: Bool, j: -2..0): Int
let k: 1..2

constraint c = fun(k, true, -1) > 10
```

- Solution:
```
let k: 1..2 = 1
let fun(i: 1..3, b: Bool, j: -2..0): Int = {
    (1, false, -2) -> 11
    (2, false, -2) -> 11
    (1, true, -2) -> 11
    (2, true, -2) -> 11
    (1, false, -1) -> 11
    (2, false, -1) -> 11
    (1, true, -1) -> 11
    (2, true, -1) -> 11
}
```
