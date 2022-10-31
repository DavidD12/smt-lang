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

constraint cst1 = r > 2.5 and j <= 5
constraint cst2 = b => j > 0
