# Scope, Memory & Ownership

## 1. Variable Scope

- **Scope** defines the region of code where a variable is valid and usable
- A variable is constrained to the **block of code** (enclosed by `{}`) where it was declared
- Variables come into scope when declared and go out of scope when the block ends
- Every function, loop, and conditional creates its own scope — you can also use bare `{}` to define one
- Accessing a variable outside its scope causes a **compiler error**:
  ```rust
  {
      let planet = "Earth";
      println!("{}", planet); // valid
  }
  println!("{}", planet); // ERROR — planet is out of scope
  ```
- To use a variable across multiple blocks, declare it in the **outer scope**

---

## 2. Shadowing Variables

- Rust allows declaring a **new variable with the same name** as an existing one using `let` — this is called **shadowing**
- The new variable masks the original for the rest of its scope:
  ```rust
  let planet = "Earth";
  let planet = "Mars"; // shadows the first
  println!("{}", planet); // "Mars"
  ```
- This is different from mutating — shadowing uses a new `let` statement and can change the **data type** or **mutability**:
  ```rust
  let planet = "Earth";   // &str
  let planet = 4;          // i32 — different type, no error
  ```
- Shadowing inside an inner block only lasts while that block is active — the original variable reappears after:
  ```rust
  let planet = "Earth";
  {
      let planet = 4;
      println!("{}", planet); // 4
  }
  println!("{}", planet); // "Earth" — shadow gone
  ```
- Be careful — **unintentional shadowing** can cause subtle bugs

---

## 3. Stack and Heap Memory

- Rust program memory is split into two regions: the **stack** and the **heap**

**Stack:**
- Stores values in **LIFO** (last in, first out) order
- Data is pushed on (added) and popped off (removed) automatically as functions are called and return
- Very **fast** to access — no searching needed
- **Limited size** (a few MB)
- Only stores values with a **known, fixed size** at compile time

**Heap:**
- Like a large warehouse — data can be stored anywhere with enough free space
- Slower than the stack — must search for space (**allocation**) and access via a **pointer**
- Supports **dynamic sizing** — data can grow and shrink at runtime
- Much **larger** than the stack
- A pointer (fixed size) stored on the stack points to the data's location on the heap

---

## 4. String Data Type

- Two ways to work with strings in Rust:
  - **String literal** (`&str`): hard-coded in the executable, immutable, stored in memory at compile time
  - **String type** (`String`): stored on the heap, mutable, can be dynamically created and resized

- Create a `String` from a literal:
  ```rust
  let message = String::from("Earth");
  ```
- The stack holds a **pointer**, **length**, and **capacity** for the heap-stored string data
- To modify a `String`, declare it as mutable:
  ```rust
  let mut message = String::from("Earth");
  message.push_str(" is home");
  ```
- The pointer, length, and capacity on the stack are automatically updated when the string grows
- String literals are immutable and must be known at compile time — use `String` for dynamic/runtime values

---

## 5. Ownership

- Rust manages heap memory through a system of **ownership** — no garbage collector, no manual `malloc`/`free`
- Three rules:
  1. Every value has **one and only one owner** (a variable)
  2. When the owner goes **out of scope**, the value is **dropped** and memory is freed
  3. There can only be **one owner at a time**
- This is checked entirely at **compile time** — no runtime overhead
- Prevents common bugs like memory leaks and invalid memory access (unlike C/C++)
- The trade-off: requires **thinking differently** about how data flows through a program

---

## 6. Moving, Cloning, and Copying Data

**Heap data (e.g. `String`) — Move:**
- Assigning a heap-based variable to another **transfers ownership** (called a **move**):
  ```rust
  let s1 = String::from("Mercury");
  let s2 = s1; // s1 is now invalid — ownership moved to s2
  ```
- Using `s1` after the move causes a **compiler error**
- To make an independent copy of heap data, use **clone** (explicit deep copy):
  ```rust
  let s2 = s1.clone(); // s1 and s2 are separate independent strings
  ```

**Stack data (e.g. integers) — Copy:**
- Stack-only types (integers, floats, booleans, chars, tuples of stack types) are **copied implicitly**:
  ```rust
  let x = 5;
  let y = x; // x is still valid — value was copied
  ```
- No need to clone — copying is cheap and automatic for stack data
- Types that implement the `Copy` trait behave this way

---

## 7. Transferring Ownership

- Ownership also transfers when passing values to functions as arguments

**Stack types — copied on pass:**
  ```rust
  fn process(val: i32) { ... }
  let fuel = 1;
  process(fuel);
  println!("{}", fuel); // still valid — a copy was passed
  ```

**Heap types — moved on pass:**
  ```rust
  fn process(s: String) { ... }
  let fuel = String::from("RP-1");
  process(fuel);
  println!("{}", fuel); // ERROR — ownership moved into the function
  ```
- When the function ends, the parameter goes out of scope and the heap data is **dropped**

**Solutions:**
- **Clone** before passing to keep the original:
  ```rust
  process(fuel.clone());
  ```
- **Return ownership** back from the function:
  ```rust
  fn process(s: String) -> String {
      s // return ownership back to caller
  }
  let fuel = process(fuel); // re-assign to reclaim ownership
  ```
- A function can also return a **different** string — the original input will be dropped when the function ends