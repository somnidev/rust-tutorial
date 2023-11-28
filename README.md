# Rust Tutorial

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

