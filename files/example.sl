let b: Bool
let i: Int = j + 1 // single line comment
let r: Real
/* multi 
   lines
   comment
*/
let bb: Bool = not b 
let j: Int
let rr: Real = i / 10

let k: -10..100 = i
// let f(i: Int, b: Bool, r: Real, j: 1..10): Bool = false


constraint cst1 = r > 2.5 and not (j <= 5)
constraint cst2 = b => j > 0
