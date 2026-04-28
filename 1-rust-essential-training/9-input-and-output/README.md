# CLI Arguments & File I/O

## 1. Command-Line Arguments

- Use the `args` function from `std::env` to read command-line arguments — import it first:
  ```rust
  use std::env;
  ```
- `env::args()` returns an **iterator** of argument strings — the first is traditionally the path to the executable
- Iterate with `enumerate()` to get indices alongside values:
  ```rust
  for (i, arg) in env::args().enumerate() {
      println!("{}: {}", i, arg);
  }
  ```
- Pass arguments after `cargo run`:
  ```
  cargo run moon 1969 --flag
  ```
- Access a specific argument by index using `.nth()`:
  ```rust
  let arg2 = env::args().nth(2).unwrap();
  ```
- Arguments are always **strings** — parse them if you need a numeric type
- If a required argument is missing, `.unwrap()` will **panic** — always validate argument count first:
  ```rust
  if env::args().len() <= 2 {
      println!("Not enough arguments.");
      return;
  }
  ```
- Flags (e.g. `--flag`) are just strings with a hyphen prefix — your program is responsible for interpreting them
- For more complex argument parsing, collect into a `Vec` and use `match` (covered later in the course)

---

## 2. Reading from Files

- Use the `fs` module from the standard library — not in the prelude, so import it:
  ```rust
  use std::fs;
  ```
- Read an entire file into a `String`:
  ```rust
  let contents = fs::read_to_string("planets.txt").unwrap();
  println!("{}", contents);
  ```
- `read_to_string` returns a **`Result`** enum — use `.unwrap()` for now (proper error handling covered later)
- To process the file **line by line**, use the `.lines()` method on the string:
  ```rust
  for line in contents.lines() {
      println!("{}", line);
  }
  ```
- To read a file as **raw bytes** (e.g. images, video):
  ```rust
  let bytes = fs::read("file.png").unwrap(); // returns Vec<u8>
  println!("{:?}", bytes);
  ```
- `read` and `read_to_string` are convenience wrappers around lower-level `File::open` and `read_to_end` — see the docs for more control
- For cross-platform path handling, explore `std::path`

---

## 3. Writing to Files

- Write a string to a file in one call using `fs::write`:
  ```rust
  use std::fs;
  fs::write("speech.txt", speech).unwrap();
  ```
- **Overwrites** the file completely if it already exists
- To **append** to an existing file, use `OpenOptions`:
  ```rust
  use std::fs::OpenOptions;
  use std::io::prelude::*;

  let mut file = OpenOptions::new()
      .append(true)
      .open("planets.txt")
      .unwrap();
  ```
- Write bytes to the opened file handle — prefix string literals with `b` to treat them as byte arrays:
  ```rust
  file.write(b"\nPluto").unwrap();
  ```
- The `io::prelude::*` import brings in the `Write` trait (and others) needed to call `.write()` on a file handle
- `fs::write` is a convenience wrapper around `File::create` and `write_all` — see docs for lower-level control