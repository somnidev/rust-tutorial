# Rest tutorial

## Introduction

Based upon [Rust Tutorial by Tech with Tim](https://www.youtube.com/watch?v=gvgBUY8iNO4&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&index=2). 

## Install Rust

It looks like you’re running macOS, Linux, or another Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This is the recommended way to install Rust.


```bash
source "$HOME/.cargo/env"
```

If you've installed rustup in the past, you can update your installation by running `rustup update`.

More information about Rust installation on Mac can be found [here](https://www.rust-lang.org/tools/install).

## Hello world in Rust

```rust
fn main() {
   println!("Hello world!");
}
```

## Rust Tools


### Cargo

Create a new Rust project using Cargo.

```bash
cargo new tutorial01
```

Use Cargo to compile the project.

```bash
cargo build
```

Now you can run the program.

```bash
./target/debug/tutorial01
```

Or you can simply do all that stuff using one command.

```bash
cargo run
```

You can use the `cargo check` command if you want to check if the source code compiles without errors.

Cleanup and remove `target` directory.

```bash
cargo clean
```

### Rustfmt

If you wanna format your code, you can use `rustfmt`.

```bash
rustfmt src/main.rs
```

## Variables, Constants and Shadowing

### Create a variable

The first thing we need to understand
here is that rust is a statically
and strongly typed programing language.
What that means
is that when you define a variable, it's
going to be given a type.

We have implicit and explicit types.

```rust
fn main() {
    let x = 4;
    println!("x is: {}", x);
}
```

### Variables are immutable

But we cannot reasign `x`.

```rust
fn main() {
    let x = 4;
    println!("x is: {}", x);
    x = 5;
}
```

Now let's run that.

```bash
cargo run
   Compiling tutorial01 v0.1.0 (/Users/me/dev/src/rust/rust-tutorial/tutorial01)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 4;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("x is: {}", x);
4 |     x = 5;
  |     ^^^^^ cannot assign twice to immutable variable
```

Now we get an error.

Anyways, the reason we're getting this error
is because by default in rust, all variables that we define are immutable.
Now, immutable just means we cannot change them.

### Reassgning a variable

Strange, but you can create a new variable with the same name `x`.

```rust
fn main() {
    let mut x = 4;
    println!("x is: {}", x);
    let x = x + 1;
    println!("x is: {}", x);
}
```

### Shadowing

Nested scopes allow to shadow a variable.

```rust
fn main() {
    let mut x = 4;
    println!("x is: {}", x);
    {
        let x = 2;
        println!("x is: {}");
    }
    let x = x + 1;
    println!("x is: {}", x);
}
```

### Constants

```rust
fn main() {
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}
```
 
