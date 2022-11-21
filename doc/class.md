## Classes

- Example:
```
class A {
    a: Int
}
inst a1, a2: A

class B extends A {
    x: A
    m(x: A): Int = self.a
}
inst b1, b2: B

let x: A
let y: B

constraint c = (x.a = y.m(x))
```

- Solution:
```
class B {
    inst b1 {
        a: Int = 0
        x: A = a1
        m(x: A): Int {
            (a1) -> 0
            (a2) -> 0
            (b1) -> 0
            (b2) -> 0
        }
    }
    inst b2 {
        a: Int = 0
        x: A = a1
        m(x: A): Int {
            (a1) -> 0
            (a2) -> 0
            (b1) -> 0
            (b2) -> 0
        }
    }
}
class A {
    inst a1 {
        a: Int = 0
    }
    inst a2 {
        a: Int = 0
    }
}
let x: A = a1
let y: B = b1
```