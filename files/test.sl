struct S {
    a: Int
    // b: Int // = self.a + 1
    m(s: S): S = s
    n(s: S): S = s
}

// struct S2 {
//     a: Int
//     // b: Int // = self.a + 1
//     m(s: S): S = s
//     n(s: S): S = s
// }
// inst s2: S2
inst s1: S

let x: S = s1
let f(s: S): S

class Person {
    age: Int
}
class Student extends Person {
}

inst p1: Person
inst a1, a2: Student

let i: Int = p1.age + a1.age


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
