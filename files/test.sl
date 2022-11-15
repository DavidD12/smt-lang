class A {
    a: Int
    m(x: A): Bool
}
inst a1, a2: A

class B extends A {}
inst b1, b2: B

// let f(x: A): Bool

let a: A
let b: B

constraint C1 = (
    // a1.m(b)
    b.m(b) /= a.m(a)
)