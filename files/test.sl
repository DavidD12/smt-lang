include "files/test2.sl"

var b: Bool = true
var i: Int = 10
var r: Real = 0.1

var f: Real = r
var bb: Bool = b => true or false and not b

var ii: Real = (i + 1 * 2 - 3) / (-1 +2)

constraint toto = bb and ii > 0.0 and b