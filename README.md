# KavaScript

Simple, dynamically-typed toy programming language for fun/didactic/tutorial purposes.
Implemented in Rust. The codebase is intentionally kept small, easy to read and is well-commented.

I wrote this to demonstrate how easy it is to roll your own simple and readable top-down recursive
descent parser that will handle a grammar similar to C/JS/Rust/Python or Lisp. I also wanted to show
that the old school distinction between parsing and lexing/tokenization is kind of
arbitrary and doesn't really need to exist.

This project is also an experiment to see how far I could get with the idea of parsing source code
directly into bytecode without generating an AST first. Parsing directly to bytecode can produce a
faster parser but is probably not ideal for languages that want to do complex static analysis
(e.g. type analysis). It should be relatively easy to modify this parser to generate an AST instead
of bytecode if desired.

Features:
- Syntax inspired by JS and C
- Minimalistic by design
- Top down recursive descent parser with support for infix expressions
- No separation between parsing and tokenization/lexing
- Source code is parsed into bytecode directly, without building an AST
- Token-threaded, stack-based bytecode interpreter
- Simple mark & sweep garbage collector

Limitations:
- Currently has no arrays or objects

## Installation

Clone this repository:

```
git clone git@github.com:maximecb/kavascript.git
```

Build and run the project:

```
# Run tests
cargo test

# Compiles project to ./target/debug/ksvm
carbo build

# For best performance, builds to ./target/release/ksvm
carbo build --release

# Run an example script
cargo run example.pls
```
