## Structure Definition

- Example:
```
struct S {
    a: Int
    m(i: 0..2, s: S): Int
}
inst s1, s2: S

let x, y: S

constraint c = (
    (x != y) and (x.a = s1.a) and (x.m(1, y) > 10)
)
```

- Solution:
```
struct S {
    inst s1 {
        a: Int = 0
        m(i: 0..2, s: S): Int {
            (0, s1) -> 11
            (1, s1) -> 11
            (0, s2) -> 11
            (1, s2) -> 11
        }
    }
    inst s2 {
        a: Int = 0
        m(i: 0..2, s: S): Int {
            (0, s1) -> 11
            (1, s1) -> 11
            (0, s2) -> 11
            (1, s2) -> 11
        }
    }
}
let x: S = s2
let y: S = s1
```
