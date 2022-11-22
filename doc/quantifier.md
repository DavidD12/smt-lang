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

let i: Int = sum x: 1..10 | x end
let r: Real = sum x: 1..10 | 1.0 end

let j: Int = prod x: 1..10 | x end
let s: Real = prod x: 1..10 | 1.0 end

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
            (10) -> 20
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
            (10) -> 20
        }
    }
}
let x: A = b1
let y: A = b1
let i: Int = 55
let r: Real = 10
let j: Int = 3628800
let s: Real = 1
```

- Example:
```
let k: Int = 1 + (1.0 as Int)
let t: Real = 1.0 + (1 as Real)
```

- Solution
```
let k: Int = 2
let t: Real = 2
```
