# Rust Learning Roadmap

This roadmap follows [The Rust Programming Language](https://doc.rust-lang.org/book/) and the official Rust docs to guide you step-by-step through Rust concepts with examples.

---

## 📚 Table of Contents

1. [Getting Started](#getting-started)
2. [Common Programming Concepts](#common-programming-concepts)
   - [Variables & Mutability](#variables--mutability)
   - [Constants](#constants)
   - [Shadowing](#shadowing)
   - [Data Types](#data-types)
     - [Scalar Types](#scalar-types)
     - [Compound Types](#compound-types)
   - [Functions](#functions)
   - [Comments](#comments)
   - [Control Flow](#control-flow)
3. [Understanding Ownership](#understanding-ownership)
   - [Ownership Rules](#ownership-rules)
   - [References & Borrowing](#references--borrowing)
   - [Mutable References](#mutable-references)
   - [Dangling References](#dangling-references)
   - [The Slice Type](#the-slice-type)
4. [Structs](#structs)
   - [Defining Structs](#defining-structs)
   - [Creating Instances](#creating-instances)
   - [Updating Structs](#updating-structs)
   - [Tuple Structs](#tuple-structs)
   - [Methods & Associated Functions](#methods--associated-functions)
5. [Enums & Pattern Matching](#enums--pattern-matching)
   - [Defining Enums](#defining-enums)
   - [Option](#option)
   - [Match](#match)
   - [If Let](#if-let)
6. [Modules & Packages](#modules--packages)
   - [Packages & Crates](#packages--crates)
   - [Defining Modules](#defining-modules)
   - [`use` Keyword](#use-keyword)
   - [Re-exporting](#re-exporting)
   - [Module Hierarchy](#module-hierarchy)
7. [Collections](#collections)
   - [Vectors](#vectors)
   - [Strings](#strings)
   - [Hash Maps](#hash-maps)
8. [Error Handling](#error-handling)
   - [panic!](#panic)
   - [Result](#result)
   - [Propagating Errors](#propagating-errors)
   - [`?` Operator](#operator)
9. [Generics](#generics)
   - [Functions with Generics](#functions-with-generics)
   - [Structs & Enums with Generics](#structs--enums-with-generics)
   - [Trait Bounds](#trait-bounds)
   - [`impl Trait`](#impl-trait)
10. [Traits & Lifetimes](#traits--lifetimes)
    - [Defining Traits](#defining-traits)
    - [Implementing Traits](#implementing-traits)
    - [Default Implementations](#default-implementations)
    - [Lifetimes](#lifetimes)
11. [Testing](#testing)
    - [Unit Tests](#unit-tests)
    - [Integration Tests](#integration-tests)
    - [Test Organization](#test-organization)
12. [Advanced Topics](#advanced-topics)
    - [Unsafe Rust](#unsafe-rust)
    - [Smart Pointers](#smart-pointers)
    - [Concurrency](#concurrency)
    - [Macros](#macros)

---

## Getting Started

### Installing Rust (`rustup`)

Install Rust using the official installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

````

### Your First Program

Create a new project:

```bash
cargo new hello_world
cd hello_world
````

Edit `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Build and run:

```bash
cargo run
```

---

## Common Programming Concepts

### Variables & Mutability

```rust
let x = 5;
println!("x = {}", x);
let mut y = 10;
y = 20;
println!("y = {}", y);
```

### Constants

```rust
const MAX_POINTS: u32 = 100_000;
println!("Max points: {}", MAX_POINTS);
```

### Shadowing

```rust
let x = 5;
let x = x + 1;
let x = x * 2;
println!("x = {}", x); // 12
```

### Data Types

#### Scalar Types

```rust
let a: i32 = -10;
let b: u32 = 10;
let c: f64 = 3.14;
let d: char = 'z';
let e: bool = true;
```

#### Compound Types

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let arr = [1, 2, 3, 4, 5];
```

### Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
println!("3 + 4 = {}", add(3, 4));
```

### Comments

```rust
// This is a comment
/* This is a
   multi-line comment */
```

### Control Flow

```rust
let number = 7;
if number < 5 {
    println!("less than five");
} else {
    println!("five or more");
}

for i in 1..4 {
    println!("i = {}", i);
}

let mut count = 0;
while count < 3 {
    println!("count = {}", count);
    count += 1;
}
```

---

## Understanding Ownership

### Ownership Rules

- Each value in Rust has a variable that’s its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value is dropped.

**Example:**

```rust
{
    let s = String::from("hello");
    println!("{}", s);
} // s is dropped here
```

---

### References & Borrowing

References allow you to refer to some value without taking ownership.

**Example:**

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
fn calculate_length(s: &String) -> usize {
    s.len()
}
println!("Length: {}", len);
```

---

### Mutable References

Mutable references allow you to change the value you borrow.

**Example:**

```rust
let mut s = String::from("hello");
change(&mut s);
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
println!("{}", s);
```

---

### Dangling References

Rust prevents dangling references at compile time.

**Example (will not compile):**

```rust
// let reference_to_nothing = dangle();
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
```

---

### The Slice Type

Slices let you reference a contiguous sequence of elements in a collection.

**Example:**

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..];
println!("{} {}", hello, world);
```
