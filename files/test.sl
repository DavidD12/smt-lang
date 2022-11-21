/*
let i: Int

let j: Int = i as 11..1000

let k: Int = 
    if j > 10 then j - 1 
    elif j < 10 then j + 1
    else j + 1 
    end

// let k: Int = if j > 10 then { j-1 } else { j+1 }

let l: Int =
    if i = 0 then 1
    elif i = 1 then 0
    else k
    end

// let l: Int = match l {
//     0 -> 1
//     1 -> 0
//     _ -> k
// }
*/

class A {
    i: Int
}
class B extends A {
}

inst a1: A
inst b1: B

let a: A
let b: B


// let f(i, j: 1..10, a: A, b: A): Real
let f(a: A): Int

constraint C = (
    // true // a.i = b.i
    forall a: B | f(a) = 1 end
    and 
    exists a: A | f(a) = 10 end
    // forall i: 1..10, j: 1..10, a: A, b: A | f(i, j, a, b) = 0.1 end
)

// maximize (j + 10) as 0..100
