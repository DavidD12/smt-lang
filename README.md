SMT-language is a simple input language (parsing/resolve/typing/translate/import). It's main objective is to ease the use Sat Modulo Theory solver(s) (actually z3).

# Install

1. Intall z3 prover
   1. Ubuntu
   ```console
    xxx@XXX:~$ sudo apt install z3
    ```
   2. Or see [Z3Prover](https://github.com/Z3Prover/z3)
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
let k: -10..100 = i

constraint cst1 = r > 2.5 and j <= 5
constraint cst2 = b => j > 0
```

## Solve

```console
xxx@XXX:~$ smt-lang --file example.sl
```

## Solution
```
    let b: Bool = true
    let i: Int = 2
    let r: Real = 7/2
    let bb: Bool = false
    let j: Int = 1
    let rr: Real = 1/5
    let k: -10..100 = 2
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
- -2..10: int interval

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

- set
- array
- function
- enum
- struct
- forall/exists/in expressions
- minimize/maximize/pareto
  