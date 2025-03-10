# Learning Rust
Repo to follow along "The Rust Programming Language" book

## Notes

### Cargo
- create a project using `cargo new`
- build a project using `cargo build` (`cargo build --release` to compile it with optimizations)
- build and run a project in one step using `cargo run`
- build a project without producing a binary to check for errors using `cargo check`

### Namespace Operator `::`
- Associated functions: functions defined on a type rather than an instance of a type
- Used to navigate through modules and access items within items
- Used to access static variables, constants, functions and other items that belong to a type or module itself, not to a specific instance of that type

### Member Access Operator `.`
- Call methods that operate on a specific object or instance
- Access fields of a struct
