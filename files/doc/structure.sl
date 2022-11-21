struct S {
    a: Int
    m(i: 0..2, s: S): Int
}
inst s1, s2: S

let x, y: S

constraint c = (
    (x /= y) and (x.a = s1.a) and (x.m(1, y) > 10)
)