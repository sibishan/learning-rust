# Error Handling

## 1. Unrecoverable Errors

- Rust separates runtime errors into two categories:
  - **Recoverable** — errors the program can reasonably handle (e.g. file not found)
  - **Unrecoverable** — errors that indicate a bug and must stop the program (e.g. index out of bounds)
- Rust has no exceptions — instead it uses the `panic!` macro for unrecoverable errors and the `Result` enum for recoverable ones
- Calling `panic!` immediately **terminates the program** and prints an error message:
  ```rust
  panic!("Houston, we've had a problem");
  ```
- Output includes the panic message, the source file, and the line number where it occurred
- The process exits with **code 101** on panic
- Some operations trigger a panic automatically (e.g. dividing by zero, out-of-bounds indexing)
- To see the **call stack** leading to a panic, enable the backtrace:
  ```
  $Env:RUST_BACKTRACE=1   # Windows PowerShell
  RUST_BACKTRACE=1        # Linux/macOS
  ```
  Then re-run with `cargo run` — scan the output for lines referencing your source files

---

## 2. Result\<T, E\> Enum

- The `Result<T, E>` enum represents operations that can either **succeed or fail**:
  ```rust
  enum Result<T, E> {
      Ok(T),  // success — stores the value
      Err(E), // failure — stores the error
  }
  ```
- `Result` is included in the **prelude** — no import needed
- Functions that can fail return `Result` instead of the raw value (e.g. `fs::read_to_string`)
- Use `.unwrap()` to extract the value — **panics if it's an error**:
  ```rust
  let contents = fs::read_to_string("file.txt").unwrap();
  ```
- Use `.expect("message")` to panic with a **custom error message**:
  ```rust
  let contents = fs::read_to_string("file.txt")
      .expect("Failed to open file");
  ```
- Both `.unwrap()` and `.expect()` are convenient but generally **not recommended** — prefer proper error handling

---

## 3. Matching Result\<T, E\> to Recover from Errors

- Use `match` to handle both outcomes without panicking:
  ```rust
  let contents = match fs::read_to_string("file.txt") {
      Ok(text) => text,
      Err(_) => String::from("File not found"),
  };
  ```
- To handle **different types of errors** differently, match on the error's `.kind()`:
  ```rust
  use std::io;

  let contents = match fs::read_to_string("file.txt") {
      Ok(text) => text,
      Err(e) => match e.kind() {
          io::ErrorKind::NotFound => String::from("File not found"),
          io::ErrorKind::PermissionDenied => String::from("Permission denied"),
          _ => panic!("Unexpected error: {:?}", e),
      },
  };
  ```
- `io::ErrorKind` is part of `std::io` — import it at the top of your program
- This approach gives your program the chance to **recover gracefully** rather than crash

---

## 4. Propagating Errors

- Instead of handling an error inside a function, you can **propagate** it upward to the caller where more context is available:
  ```rust
  fn read_and_combine(path1: &str, path2: &str) -> Result<String, io::Error> {
      let text1 = match fs::read_to_string(path1) {
          Ok(t) => t,
          Err(e) => return Err(e), // propagate error up
      };
      // ...
      Ok(combined)
  }
  ```
- The **`?` operator** is shorthand that does the same thing — extract the value if `Ok`, or return the error immediately if `Err`:
  ```rust
  fn read_and_combine(path1: &str, path2: &str) -> Result<String, io::Error> {
      let text1 = fs::read_to_string(path1)?; // returns error if it fails
      let text2 = fs::read_to_string(path2)?;
      Ok(text1 + &text2)
  }
  ```
- The `?` operator can **only be used in functions that return `Result`**
- The error type of the propagated result must **match** the function's declared error type — mixing different error types requires additional handling (see Rust By Example: multiple error types)
- The caller then handles the result with a match expression:
  ```rust
  match read_and_combine("a.txt", "b.txt") {
      Ok(text) => println!("{}", text),
      Err(e) => println!("Error: {}", e),
  }
  ```