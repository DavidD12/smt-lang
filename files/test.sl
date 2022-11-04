let a: 1..2

struct S {
    a: Int = a
    i: 1..10
//    b: Bool = self.a > a + i
    m(b: Bool, s: S): S
    other: S
    meth(b: Bool, i: 1..0, r: Real): Int = a
}

let s: S

/*
constraint C1 = (
    s.a + s.i > 10
)

constraint C2 = (
    s.m(true, s) = s
)
*/