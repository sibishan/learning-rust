# 1. Write Your First Program

## 1. Installing Rust on Linux and macOS

- Visit [rust-lang.org](https://rust-lang.org) and copy the shell command from the install page
- Run the command in your terminal — it downloads and installs `rustup`
- Select option 1 to proceed with the default installation
- Add Cargo to your PATH for the current shell:
  ```bash
  source $HOME/.cargo/env
  ```
- Verify installation: `rustc --version`
- Update Rust anytime: `rustup update`
- Open local docs: `rustup doc`

---

## 2. Anatomy of a Rust program

- Rust source files use the `.rs` extension (e.g. `main.rs`) — no spaces in filenames
- Every program needs a `main` function as the entry point:
  ```rust
  fn main() {
      println!("Hello, world!");
  }
  ```
- `println!` is a **macro** (indicated by `!`) — prints text followed by a newline
- Strings are wrapped in double quotes `"..."`
- Most statements end with a **semicolon** `;`
- Rust is a **free-form language** — spacing doesn't affect compilation, but use **4 spaces** for indentation by convention
- Compile with: `rustc main.rs` → run with `./main` (or `.\main.exe` on Windows)

---

## 3. Adding comments

- **Single-line comment:** `// your comment here`
- **Inline comment:** placed at the end of a line of code
- **Multi-line comment:**
  ```rust
  /* This is a
     multi-line comment */
  ```
- The compiler ignores all comment content
- **Don't over-comment** — avoid stating the obvious; only comment on non-apparent logic or design decisions

---

## 4. Building programs using Cargo

- Cargo is Rust's built-in **package manager and build tool** (installed alongside Rust)
- Check version: `cargo --version`
- Create a new project:
  ```bash
  cargo new hello_world
  ```
- Project structure:
  - `Cargo.toml` — project config (name, version, edition, dependencies)
  - `src/main.rs` — your source code goes here
  - `Cargo.lock` — auto-generated, do **not** manually edit
  - `target/` — build output directory
- Build and run: `cargo run`
- Release build (optimised): `cargo build --release`
- External libraries are called **crates** and are listed under `[dependencies]` in `Cargo.toml`