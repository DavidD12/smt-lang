- Example:
```
class A {
    a: Int
}
class B extends A {
    m(i: 1..10): Int
}
inst b1, b2: B

let x, y: A

constraint c = (
    forall e: A | e.a > 10 end
    and
    exists e: B, i: 1..10 | e.m(i) = 20 end 
    and
    x.a = if x = b1 then 1 elif x = b2 then 2 else 3 end
)
```

- Solution:
```
class A {
}
class B {
    inst b1 {
        a: Int = 11
        m(i: 1..10): Int {
            (1) -> 20
            (2) -> 20
            (3) -> 20
            (4) -> 20
            (5) -> 20
            (6) -> 20
            (7) -> 20
            (8) -> 20
            (9) -> 20
        }
    }
    inst b2 {
        a: Int = 11
        m(i: 1..10): Int {
            (1) -> 20
            (2) -> 20
            (3) -> 20
            (4) -> 20
            (5) -> 20
            (6) -> 20
            (7) -> 20
            (8) -> 20
            (9) -> 20
        }
    }
}
let x: A = b1
let y: A = b1
```