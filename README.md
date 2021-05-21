# Plush

**Please note that Plush is at the experimental stage. You are likely to encounter bugs
and breaking changes in the language.***

Minimalistic, dynamically-typed programming language for fun/didactic/tutorial purposes. Implemented in Rust.

Features/Goals:
- Syntax inspired from JS, Wren, Rust
- Simple object model based on prototypal inheritance
- Immutable strings
- Variales and object fields are immutable by default
- Token-threaded, stack-based interpreter
- JIT based on Lazy Basic Block Versioning (LBBV)
- Fast single-generation copying GC

Design principles:
- The syntax should seem familiar and easy to understand to most
  - Assuming some familiarity with JS or Python
- The language semantics should be intuitive
  - Avoid undefined behaviors as much as possible
  - Avoid special cases, corner cases and hidden "automagic" behavior
- Keep the language intentionally small and easy to learn
- Aim for code that is well-commented and easy to read
  - Avoid one-liners and clever hacks
  - Try to make it easy for beginners to approach
- Minimize dependencies and in so doing, risk of code breakage

## Installation

Clone this repository:
git clone git@github.com:maximecb/plush.git

```
# Builds and runs the project
cargo run

# Compiles project to ./target/debug/plush
carbo build

# For best performance, builds to ./target/release/plush
carbo build --release
```
