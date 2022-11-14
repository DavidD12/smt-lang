class A {
    a: Int
}
inst a1, a2: A

class B extends A {}
inst b1, b2: B

let a: A
let b: B = b1

constraint C1 = (
    a.a = 10
    and b.a = 0
)