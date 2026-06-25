# Writing an Interpreter in Go (in Rust)

This is a toy interpreter for the made up "Monkey" language that I built while reading Thorsten Ball's book, [*Writing an Interpreter in Go*](https://interpreterbook.com/).

This version is written in Rust. It includes a lexer, Pratt parser, AST, evaluator, object system, environments, a small REPL, and tests for the implemented language pieces.

## Requirements

- Rust 2021 edition
- Cargo

## Running

Start the REPL with:

```sh
cargo run
```

The REPL will then look something like this:

```monkey
> let add = fn(x, y) { x + y };
fn(x,y,) {x}
> add(2, 3);
5
> if (1 < 2) { "yes" } else { "no" };
yes
```

Use `Ctrl-C` to exit the REPL.

Run the test suite with:

```sh
cargo test
```

## Language Features

| Feature | Support | Notes |
| --- | --- | --- |
| Integer literals | Full | 32-bit signed integers. |
| Boolean literals | Full | `true` and `false`. |
| String literals | Full | Double-quoted strings. Escape sequences are not implemented. |
| Prefix operators | Full | `!` and unary `-`. |
| Integer arithmetic | Full | `+`, `-`, `*`, `/`, and exponentiation with `**`. |
| Comparisons | Full | `==`, `!=`, `<`, and `>`. |
| Operator precedence | Full | Includes grouped expressions with parentheses. |
| `if` / `else` expressions | Full | Conditions use Monkey truthiness; `0` and `null` are falsey. |
| `return` statements | Full | |
| `let` bindings | Full | Bind values in the current environment. |
| Identifiers | Full | Supports lookup from local and parent environments. |
| Function literals | Full | `fn(...) { ... }`. |
| Function calls | Full | User-defined functions and builtins can be called. |
| Closures / enclosed environments | Partial | Function calls create enclosed environments, but function cloning drops the captured environment. |
| String concatenation | Partial | `"hello " + "world"` works; other string operators are not implemented. |
| Builtins / standard library | Partial | `import "std.string";` exposes `strlen(...)`, but no other standard library functions have been implemented. |
| Imports | Partial | Only `import "std.string";` has behavior. Other namespaces are ignored. |
| Arrays | Partial | Bracket tokens and an AST node exist, but array literals and indexing are not parsed or evaluated. |
| File execution | Not supported | The binary currently runs only the REPL. |

## Example Session

```monkey
> let double = fn(x) { x * 2 };
fn(x,) {x}
> double(21);
42
> import "std.string";
null
> strlen("monkey");
6
> "hello " + "monkey";
hello monkey
```
