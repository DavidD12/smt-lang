
let b: Bool
let i: 1..5
let r: Real = i / g(true, i)

let f(j: 1..10): Bool = j > i
let g(b: Bool, i: 1..5): Int

constraint C1 = f(i + 1)
constraint C2 = (g(b, i) > i) or f(i) and not f(i+1)