# The Rust Programming Language

This will summarize all key concept I will encounter when reading this book.
I decided to maintain this repository because I found rust very interesting,
it is different from every programming language I have ever tried.
I this this learning experience will be a lot of fun

## Installation

**Rustup** is the installer for the rust programming language, it is like
nvm for those are used to Javascript

- On **macOS** and **Linux** can be installed via this script:
`curl https://sh.rustup.rs -sSf | sh`
- On **windows** you can install it by downloading and running
[this](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
exe file

After installation Rust toolchain can be **upgraded** by using the following
command: `rustup update`  
To **uninstall** Rust and rustup: `rustup self uninstall`

## Hello World

To compile a Rust program we can use `rustc main.rs` this will create a
executable file that can be run

### Cargo

Cargo is Rust's build system and packet manager, creating a new project with
it is very simple `cargo new <app_name> --bin` or `cargo new <lib_name> --lib`

#### Useful Cargo commands

- `cargo run` runs the program without create the executable
- `cargo build` build an executable (debug mode) that you can run later
- `cargo build --release` build an executable (release mode) that you can run later
- `cargo check` check only if the program compiles
- `cargo upgrade` when in Cargo.toml you update a dependency run this
- `cargo doc --open` generate the doc for your crate so you can check which method call
