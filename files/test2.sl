let b: Bool
let i: Int
let k: Int = i + 10
let j: 0..5
let r: Real

let f(i: 1..10, b: Bool): Int

class A {
    a: Int
}
class B extends A {
}
inst a1, a2: A
inst b1, b2: B

let x, y: A

constraint c1 = (
    b and i > 10 or not (r >= 10.0) => (j = 10)
)

constraint c2 = (
    f(j + 1, true) <= 20 and i >= 10
)

constraint c3 = (
    forall e: A then e.a = 10 end
    // (if true then 1 elif false then 2 else 3 end) = 2
)

minimize (i as 0..100)
