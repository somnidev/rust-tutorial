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

## Data Types

### Primitive Data Types

#### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

### Integers

An integer is a number without a fractional component.

```bash
Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
```

Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in the following table.

```bash
Number literals	Example
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'
```

Note that number literals that can be multiple numeric types allow a type suffix, such as `57u8`, to designate the type.

Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. 

```rust
fn main() {
    let x: i32 = -16;
    println!("x is: {}", x);
    let mut y: i8 = 120;
    y = y + 55;
    println!("y is: {}", y);
}
```

If you run this code, then there is an overflow `y = y + 55" and rust shows a panic.

>Note: The default integer type in Rust is `i32`.

### Floating-Point Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

### Numeric Operations

Rust supports the basic mathematical operations you’d expect for all the number types.

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

### The Boolean Type

As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. 

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

### The Character Type

Rust’s char type is the language’s most primitive alphabetic type.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

## Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type.

```rust
fn main() {
    let tup: (i32, bool, char) = (1, true, 's');
    // println!("tup: {}", tup)
}
```

>Note: Tuples don't implement `std::fmt::Display`, that means `(i32, bool, char)` cannot be formatted with the default formatter`.

```rust
fn main() {
    let tup: (i32, bool, char) = (1, true, 's');
    println!("tup: ({},{},{})", tup.0, tup.1, tup.2)
}
```

### The Array Type

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.


```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

