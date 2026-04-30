# Standard Library, Input & Crates

## 1. Rust Standard Library and Prelude

- The **standard library** (`std`) provides core data types, functions, macros, and utilities for writing portable Rust programs
- Documentation is available locally via `rustup doc` or online at [doc.rust-lang.org](https://doc.rust-lang.org)
- Organised into **modules** (e.g. `std::thread`, `std::io`) — bring a module into scope with a `use` declaration:
  ```rust
  use std::thread;
  thread::spawn(...);
  ```
- The **prelude** is a small set of the most commonly needed items that Rust **automatically imports** into every program — this is why we can use `String`, `Vec`, `println!` etc. without any `use` statement
- If a module is not in the prelude, you must import it manually with `use`
- The prelude is intentionally kept minimal — importing more than you need is bad practice

---

## 2. Standard Input

- Standard input lets users type messages into the command line for the program to read
- The `io` module handles this — it is **not** in the prelude, so import it manually:
  ```rust
  use std::io;
  ```
- Create a mutable `String` buffer to store the input (heap-allocated so it can resize to any length):
  ```rust
  let mut buffer = String::new();
  ```
- Read a line of input from the user:
  ```rust
  println!("Enter a message:");
  io::stdin().read_line(&mut buffer);
  ```
- `read_line` **blocks execution** and waits until the user presses Enter, then fills the buffer with the input string (including a trailing newline character)
- Print the buffer to verify the input was received:
  ```rust
  println!("{}", buffer);
  ```

---

## 3. Parsing Strings

- Input from `read_line` is always a `String` — use `.parse()` to convert it to another data type
- First, **trim** the string to remove leading/trailing whitespace and the trailing newline:
  ```rust
  buffer.trim()
  ```
- Then parse into the desired type — specify the type using the **turbofish operator** or via type annotation:
  ```rust
  // Option 1: turbofish
  let number = buffer.trim().parse::<i32>();

  // Option 2: type annotation
  let number: i32 = buffer.trim().parse();
  ```
- `parse()` returns a **`Result` enum** — either the parsed value or an error (e.g. if the user types letters instead of digits)
- Use `.unwrap()` to extract the value for now (covered properly in error handling):
  ```rust
  let number = buffer.trim().parse::<i32>().unwrap();
  ```
- If the input cannot be parsed (e.g. typing `"thirteen"` instead of `"13"`), `unwrap()` will **panic** and crash the program
- Once parsed, the variable behaves as its numeric type (e.g. you can do `number + 1`)

---

## 4. Crates

- A **crate** is a collection of Rust source code files — two types:
  - **Binary crate**: compiles to an executable (every program we've written is one)
  - **Library crate**: a reusable collection of code for use in other programs
- **[crates.io](https://crates.io)** is the official community registry for third-party crates
- To use an external crate (e.g. `rand` for random number generation), add it to `Cargo.toml`:
  ```toml
  [dependencies]
  rand = "0.8.0"
  ```
- Cargo will automatically **download, compile, and link** the crate and its dependencies on the next build
- Import the crate into your program with a `use` statement:
  ```rust
  use rand;
  let n: f64 = rand::random();
  ```
- Import a specific function directly to avoid writing the full path every time:
  ```rust
  use rand::random;
  let n: f64 = random();
  ```
- Use a **wildcard** `*` to import everything from a crate's prelude:
  ```rust
  use rand::prelude::*;
  ```
- Use the **turbofish operator** to tell a generic function what type to produce:
  ```rust
  let n: f64 = rand::random::<f64>();
  ```
- Generate a random number within a range using `thread_rng()` and `gen_range()`:
  ```rust
  let number = thread_rng().gen_range(1..11); // 1 to 10 inclusive
  ```
- Avoid importing a function with the same name as one you define yourself — it causes a **naming conflict**