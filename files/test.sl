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

maximize (j + 10) as 0..100
