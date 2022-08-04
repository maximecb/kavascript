# Plush

Simple, dynamically-typed toy programming language for fun/didactic/tutorial purposes.
Implemented in Rust. The codebase is intentionally kept small, easy to read and is well-commented.

Features:
- Syntax inspired by JS and C
- Simple and minimalistic by design
- Top down recursive descent parser
- Source code is parsed into bytecode directly, without building an AST
- Token-threaded, stack-based bytecode interpreter

Limitations:
- This language has no GC and everything is passed by value

## Installation

Clone this repository:

```
git clone git@github.com:maximecb/plush.git
```

Build and run the project:

```
# Run tests
cargo test

# Compiles project to ./target/debug/plush
carbo build

# For best performance, builds to ./target/release/plush
carbo build --release

# Run an example script
cargo run example.pls
```
