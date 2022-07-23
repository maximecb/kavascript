# Plush

Minimalistic, dynamically-typed toy programming language for fun/didactic/tutorial purposes. Implemented in Rust.
The source code is intentionally kept simple and minimalistic, and is well-commented.

Features:
- Syntax inspired by JS and C
- Top down recursive descent parser
- Plush code is parsed into bytecode directly, without an AST

## Installation

Clone this repository:

```
git clone git@github.com:maximecb/plush.git
```

Build and run the project:

```
# Run tests
cargo test

# Builds and runs the project
cargo run

# Compiles project to ./target/debug/plush
carbo build

# For best performance, builds to ./target/release/plush
carbo build --release
```
