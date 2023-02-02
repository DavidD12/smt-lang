let x: 1..10
let y, z: Int

constraint c0 = x >= 2
constraint c1 = y >= 1
constraint c3 = z >= 3

constraint c = (max(x, y, z) <= 5 and max(x, y, z) >= 3)

minimize ((x + y + z) as 0..100)
