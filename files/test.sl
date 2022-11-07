struct S {}

inst s1, s2: S

let x, y: S
let z: S = s1

constraint C1 = (
    x = y   
)