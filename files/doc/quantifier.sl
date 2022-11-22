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