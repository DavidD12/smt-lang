struct S {
    a: Int
    // b: Int // = self.a + 1
    m(s: S): S
}
inst s1, s2: S
// inst s1, s2: S

let x: S = s1

let f(s: S): S

// let x, y: S
// let z: S // = s1

// constraint C1 = (
//     x = y
// )

// constraint C2 = (
//     z.a = 1
//     and z.m(10) = 20
//     // and s2.b = 10
// )
