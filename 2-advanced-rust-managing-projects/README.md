# Advanced Rust: Managing Projects & Testing

---

## Part 1: The Module System

### 1.1 The Module System Overview

- The **module system** organises large Rust programs for readability, reuse, and privacy control
- Four key features:
  - **Modules** — subdivide and group related code; control scope and privacy
  - **Paths** — name and refer to items within the module tree (absolute or relative)
  - **Crates** — binary (executable) or library (reusable code); can refer to source, compiled artifacts, or packages from crates.io
  - **Packages** — contain one or more crates; described by a `Cargo.toml` manifest file
- A package can have **at most one library crate** and **any number of binary crates**

---

### 1.2 Packages vs. Crates

- Create a new package: `cargo new my_project`
- Package structure:
  - `Cargo.toml` — manifest file with name, version, edition, and dependencies
  - `src/main.rs` — root of the **binary crate** (must contain `fn main()`)
  - `src/lib.rs` — root of the **library crate** (no `main` needed)
  - `src/bin/` — additional binary crates; each `.rs` file compiles to its own executable
- A crate in `src/bin/` that needs its own modules uses a sub-directory named after the crate, with `main.rs` inside it
- Build all crates in a package: `cargo build --workspace`
- Output goes to `target/debug/` — executables and `.rlib` for library crates
- Optional package directories: `benches/`, `examples/`, `tests/`

---

### 1.3 Defining Modules

- Define an **inline module** using the `mod` keyword:
  ```rust
  mod hello {
      pub fn english() { println!("Hello"); }
      pub fn spanish() { println!("Hola"); }

      pub mod casual {
          pub fn english() { println!("Hey"); }
      }
  }
  ```
- Modules create isolated **namespaces** — two functions can share the same name if they are in different modules
- Modules can contain functions, structs, enums, traits, impl blocks, constants, and other modules

---

### 1.4 Absolute vs. Relative Paths

- Paths use `::` to separate layers of the module hierarchy
- **Absolute path** — starts from the crate root using the crate name or the keyword `crate`:
  ```rust
  crate::hello::casual::english();
  ```
- **Relative path** — starts from the current scope:
  ```rust
  hello::spanish(); // from the crate root
  casual::english(); // from within the hello module
  ```
- Use `super` to build a relative path starting from the **parent module**:
  ```rust
  super::spanish(); // calls spanish from within the casual submodule
  ```
- Prefer relative paths — absolute paths break if you rename a module

---

### 1.5 Public Modules

- All items in a module are **private by default** — parent modules cannot access private children
- Child modules **can** access private items from their parent/ancestor modules
- Make an item public with the `pub` keyword:
  ```rust
  pub fn english() { ... }
  ```
- A **module itself** must also be declared `pub` for external code to reach items inside it:
  ```rust
  pub mod casual { ... }
  ```

---

### 1.6 Public Structs and Enums

- Marking a **struct** as `pub` makes it visible, but its fields remain **private by default** — each field must be individually marked `pub`:
  ```rust
  pub struct Rectangle {
      pub width: f64,  // accessible outside
      height: f64,     // still private
  }
  ```
- Marking an **enum** as `pub` automatically makes **all its variants** public — no need to annotate each one

---

### 1.7 Bringing Paths into Scope

- Use `use` to bring a path into scope and avoid writing the full path each time:
  ```rust
  use crate::greeting::formal;
  formal::english();
  ```
- Convention: import the **parent module** for functions (keeps path context clear), but import the **item itself** for structs and enums
- Rename imports with `as` to avoid name conflicts:
  ```rust
  use std::io::Result as IoResult;
  use std::fmt::Result as FmtResult;
  ```
- Use **nested paths** to combine multiple imports from the same root:
  ```rust
  use crate::greeting::{formal, casual};
  ```
- Use the **glob operator** `*` to bring all public items from a path into scope (use sparingly):
  ```rust
  use rand::prelude::*;
  ```

---

### 1.8 Using External Crates

- Add a crate to `Cargo.toml` under `[dependencies]`:
  ```toml
  rand = "0.8.5"
  ```
- Cargo automatically downloads, compiles, and links the crate and its dependencies on the next build
- Bring items into scope with `use`:
  ```rust
  use rand::thread_rng;
  use rand::Rng;
  ```
- Use the glob on a crate's prelude (when available) to import common items:
  ```rust
  use rand::prelude::*;
  ```

---

### 1.9 Separating Modules into Multiple Files

- **Style 1 (recommended):** place the module's code in a file with the same name as the module at the same directory level:
  ```
  src/
    main.rs        ← declares `mod greeting;`
    greeting.rs    ← contents of greeting module
    greeting/
      casual.rs    ← contents of greeting::casual submodule
  ```
- **Style 2 (alternative):** use a `mod.rs` file inside a directory named after the module:
  ```
  src/
    main.rs
    greeting/
      mod.rs       ← contents of greeting module
      casual.rs    ← contents of greeting::casual submodule
  ```
- In both styles, the parent file declares the module with `mod greeting;` — the compiler finds the corresponding file automatically
- Both styles can coexist in the same project

---

## Part 2: Testing

### 2.1 Test Functions

- Annotate a function with `#[test]` to mark it as a test:
  ```rust
  #[test]
  fn it_works() {
      assert_eq!(2 + 2, 4);
  }
  ```
- Run all tests with: `cargo test`
- Output shows each test name with a `ok` or `FAILED` result, followed by a summary

---

### 2.2 assert! Macro

- `assert!(expr)` — passes if `expr` is `true`, panics (fails) if `false`:
  ```rust
  assert!(is_even(42));
  assert!(!is_even(13));
  ```
- Multiple `assert!` calls in one test function will stop at the first failure — the rest won't run
- For independent pass/fail results, put each assertion in its own test function

---

### 2.3 assert_eq! and assert_ne! Macros

- `assert_eq!(left, right)` — passes if the two values are equal; shows both values on failure
- `assert_ne!(left, right)` — passes if the two values are not equal
- Both require the arguments to implement `PartialEq` and `Debug`
- Add a **custom failure message** as additional arguments (passed to `format!`):
  ```rust
  assert_eq!(result, 4, "Expected 4, got {}", result);
  ```

---

### 2.4 should_panic Attribute

- Use `#[should_panic]` to verify that code panics as expected:
  ```rust
  #[test]
  #[should_panic]
  fn test_empty_name() {
      Person::new(String::from(""));
  }
  ```
- If the function **panics** → test passes; if it **does not panic** → test fails
- Restrict to a specific panic message using the `expected` parameter:
  ```rust
  #[should_panic(expected = "needs a name")]
  ```

---

### 2.5 Controlling Test Execution

- Run all tests: `cargo test`
- Show standard output even for passing tests: `cargo test -- --show-output`
- Run a single test by name: `cargo test test_zero`
- Run all tests whose name contains a substring: `cargo test numbers`
- Run tests sequentially (no parallelism): `cargo test -- --test-threads=1`
- Ignore a test by default with `#[ignore]`; run only ignored tests: `cargo test -- --ignored`

---

### 2.6 Unit Tests

- **Unit tests** test individual modules in isolation and live in the **same file** as the code they test
- Organised into a `tests` module annotated with `#[cfg(test)]` — only compiled during `cargo test`:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn ut_add_two() {
          assert_eq!(add_two(2, 3), 5);
      }
  }
  ```
- `use super::*` brings the parent module's items (including private ones) into scope for testing
- Private functions **can** be tested directly from unit tests in the same file

---

### 2.7 Integration Tests

- **Integration tests** live in a top-level `tests/` directory (next to `src/`)
- Each `.rs` file in `tests/` is compiled as a separate test binary crate
- Must explicitly import the library being tested:
  ```rust
  use math_funcs::add_ops;
  ```
- Do **not** need `#[cfg(test)]` — Rust only compiles files in `tests/` during `cargo test`
- Run only integration tests: `cargo test --test '*'`
- Run only unit tests: `cargo test --lib`
- To share common support code across integration test files without it being treated as a test, place it in `tests/test_utils/mod.rs` (not at the top level of `tests/`)
- **Binary crates cannot be integration tested directly** — move core logic into a library crate first