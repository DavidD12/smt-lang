## Variable

### Declare variables

Variables are defined using the keyword **let**. Multiple variables can be declared in a single line with the following syntax:

```
let b: Bool
let i, j: Int
let x, y, z: Real
```

### Define variables

Variables can also be defined:
```
let i: Int = j + 20
let j: Int = 10
```

## Types

Variables can be declared with the following *primitive* types: **Bool**(boolean), **Int** (non bounded integer), **min..max** (integer bounded interval), **Real** (real, not float).

```
let b: Bool
let i: Int
let j: -100..100
let r: Real
```

## Comments

Our syntax recognize single and multi-line(s) comments:

```
// Single line comments
/*
 muti
 lines
 comments
*/
```

## Import Files

Other files can be included:

- file *"part_1.sl"*:
```
let i: Int
constraint c = (i > 0)
```

- file *"part_2.sl"*:
```
include "part_1.sl"
let j: Int = i + 1
```

- Solve:
```
smt-lang -f part_2.sl
```
- Result:
```
let j: Int = 2
let i: Int = 1
```
