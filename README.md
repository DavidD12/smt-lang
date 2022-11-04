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
/*
  Simple and Stupid Example
*/
let b: Bool
let i: 1..5
let r: Real = i / g(true, i)

let f(j: 1..10): Bool = j > i
let g(b: Bool, i: 1..5): Int

// comment
constraint C1 = f(i + 1)
constraint C2 = (g(b, i) > i) or f(i) and not f(i+1)
```

## Solve

```console
xxx@XXX:~$ smt-lang --file example.sl
```

## Solution
```
let b: Bool = false
let i: 1..5 = 1
let r: Real = 1/2
let f(j: 1..10): Bool = {
    (1) -> false
    (2) -> true
    (3) -> true
    (4) -> true
    (5) -> true
    (6) -> true
    (7) -> true
    (8) -> true
    (9) -> true
}
let g(b: Bool, i: 1..5): Int = {
    (false, 1) -> 2
    (true, 1) -> 2
    (false, 2) -> 2
    (true, 2) -> 2
    (false, 3) -> 2
    (true, 3) -> 2
    (false, 4) -> 2
    (true, 4) -> 2
}
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
  