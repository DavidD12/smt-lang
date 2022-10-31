SMT-language is a simple input language (parsing/resolve/typing/translate/import). It's main objective is to ease the use Sat Modulo Theory solver(s) (actually z3).

# Install

1. Intall z3 prover
   1. Ubuntu
   ```console
    xxx@XXX:~$ sudo apt install z3
    ```
   2. Other see [Z3Prover](https://github.com/Z3Prover/z3)
2. Install Rust: [Rust](https://www.rust-lang.org/fr)
3. Install SMT-Language:
   ```console
    xxx@XXX:~$ cargo install smt-lang
    ```

# Run SMT-language

```console
xxx@XXX:~$ smt-lang --file problem_file.sl
```

# Example

## Problem

```
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
```

## Solve

```console
xxx@XXX:~$ smt-lang --file example.sl
```

## Solution
```
var b: Bool = true
var i: Int = 2
var r: Real = 7/2
var bb: Bool = false
var j: Int = 1
var rr: Real = 1/5
```

## Options

### Verbose
- --verbose 0 : display nothing
- --verbose 1 : display analysis result
- --verbose 2 : display loaded problem
- --verbose 3 : display SMT problem and SMT model if a solution is found

# Syntax

## Type

- Bool: boolean type
- Int: integer type (unbounded)
- Real: real type

## Variable

```
let <identifier> : <Type> [= <Expression>]
```

## Constraint

```
constraint <identifier> = <Expression>
```

## Expression

```
include ".+"
```

```
true | false
not <Expression>
<Expression> and <Expression>
<Expression> or <Expression>
<Expression> => <Expression>
```

```
<Expression> = <Expression>
<Expression> /= <Expression>
<Expression> < <Expression>
<Expression> <= <Expression>
<Expression> > <Expression>
<Expression> >= <Expression>
```

```
[0-9]+ | [0-9]+.[0-9]+
- <Expression>
<Expression> + <Expression>
<Expression> - <Expression>
<Expression> * <Expression>
<Expression> / <Expression>
```

```
( <Expression> )
```

### Comming Soon:

- int interval
- set
- array
- function
- enum
- struct
- forall/exists/in expressions
- minimize/maximize/pareto
  