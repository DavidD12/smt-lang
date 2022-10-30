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

let <identifier> : <Type> [= <Expression>]

## Constraint

constraint <identifier> = <Expression>

## Expression

