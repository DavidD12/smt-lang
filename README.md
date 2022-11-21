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
let i: 1..100
let r: Real

constraint C1 = (
    i >= 10
)
constraint C2 = (
    r <= 20.0 and b
)
```

## Solve

```console
xxx@XXX:~$ smt-lang --file example.sl
```

## Solution
```
let b: Bool = true
let i: 1..100 = 10
let r: Real = 20
```

## Options

### Verbose
- --verbose 0 : display nothing except the result
- --verbose 1 : display analysis result
- --verbose 2 : display loaded problem
- --verbose 3 : display SMT problem and SMT model if a solution is found

# Syntax

- [doc/variable.md](Variables and Types and Files)
- [doc/boolean.md](Boolean Expressions)
- [doc/number.md](Integer and Real Expressions)
- [doc/function.md](Functions)
- [doc/structure.md](Structures and Instances)
- [doc/class.md](Classes and Instances)
- [doc/quantifier.md](Quantifiers Expressions)

- [doc/search.md](Search: Solve, Minimize, Maximize)