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