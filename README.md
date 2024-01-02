# rlox

Lox bytecode compiler and interpreter using only Rust's standard library.

My implementation has a few QoL improvements over the vanilla language:

- `nil` -> `null`
- `fun` -> `fn`
- `var` -> `let`
- `+=`, `-=`, `*=`, and `/=` for operation and assignment
- `%` for modulo
- `break` support inside `for` and `while` loops
- `print` as a built-in function instead of a statement
