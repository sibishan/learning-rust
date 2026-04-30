# Lifetimes

## 1. The Borrow Checker

- The **borrow checker** is a compile-time tool that validates all references by comparing the **lifetime** of variables to ensure borrows are always valid
- A variable's **lifetime** is the span from when it's declared to when it goes out of scope
- Lifetimes are labelled by convention using a tick/apostrophe prefix: `'a`, `'b`, etc.
- The borrow checker ensures that any reference used later in the program points to a value that is **still alive** at that point
- Example of a dangling reference the borrow checker catches:
  ```rust
  let propellant;
  {
      let rp1 = String::from("RP-1");
      propellant = &rp1; // rp1 goes out of scope here
  }
  println!("{}", propellant); // ERROR — rp1 no longer lives
  ```
- Fix: declare `rp1` in a wider scope so its lifetime covers where it's used

---

## 2. Lifetime Annotation Syntax

- Explicit **lifetime annotations** are required when the compiler cannot determine the lifetime of a returned reference on its own
- Lifetime parameters are declared in angle brackets after the function name — by convention single lowercase letters prefixed with `'`:
  ```rust
  fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
  }
  ```
- This tells the compiler: the returned reference will live **at least as long as the shorter of `x` and `y`**
- Annotations do **not** change how long variables actually live — they only describe the relationship between lifetimes so the borrow checker can validate them
- The borrow checker uses the **most restrictive** (shortest) lifetime among annotated inputs to validate the output

---

## 3. Multiple Lifetime Annotations

- If a function's return value can only ever come from one input, only that input needs to share a lifetime with the output:
  ```rust
  fn best_fuel<'a>(x: &'a str, y: &str) -> &'a str {
      x // always returns x — y's lifetime is irrelevant
  }
  ```
- To make it explicit that two parameters have independent lifetimes, define a second lifetime variable:
  ```rust
  fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
      x
  }
  ```
- This makes it clear that the return value's lifetime is tied to `x` only, and `y`'s lifetime is completely independent
- The borrow checker enforces exactly what the annotations describe — if you annotate `y` with `'a`, it will enforce `y`'s lifetime even if `y` is never returned

---

## 4. Lifetime Elision Rules

- Early Rust required explicit lifetime annotations on every reference — this was later relaxed with **lifetime elision rules** that let the compiler infer lifetimes in common patterns
- There are three elision rules the compiler applies in order:

**Rule 1:** Each input reference parameter gets its own distinct lifetime:
```rust
fn foo(x: &str, y: &str)  →  fn foo<'a, 'b>(x: &'a str, y: &'b str)
```

**Rule 2:** If there is exactly one input lifetime, it is used for all output lifetimes:
```rust
fn get_first_word(s: &str) -> &str  // elision applies — one input lifetime covers the output
```

**Rule 3 (methods only):** If one input is `&self` or `&mut self`, the lifetime of `self` is used for all output lifetimes:
```rust
fn send_transmission(&self, msg: &str) -> &str  // output gets self's lifetime
```

- If after applying all three rules the compiler still cannot determine all output lifetimes, it requires explicit annotations
- These rules are for the **compiler** — you don't need to follow them manually, but knowing them helps you understand when annotations can be omitted

---

## 5. Struct Lifetime Annotations

- When a struct stores a **reference** (e.g. `&str`) instead of an owned value (e.g. `String`), the compiler needs to know how the struct's lifetime relates to the borrowed data
- Declare a lifetime parameter in angle brackets after the struct name and annotate the reference field:
  ```rust
  struct Shuttle<'a> {
      name: &'a str,
  }
  ```
- The `impl` block also needs the lifetime declared:
  ```rust
  impl<'a> Shuttle<'a> {
      fn get_name(&self) -> &str {
          self.name // Rule 3 applies — self's lifetime used for output
      }
  }
  ```
- If a method returns something **other than a field from self**, explicit annotations are needed on the method:
  ```rust
  impl<'a> Shuttle<'a> {
      fn send_transmission<'b>(&'a self, msg: &'b str) -> &'b str {
          msg // returning msg, not self — needs 'b annotation
      }
  }
  ```
- A struct that owns its data (e.g. `name: String`) does not need lifetime annotations — ownership handles the lifetime automatically

---

## 6. Static Lifetime

- `'static` is a reserved lifetime name indicating data that **lives for the entire duration of the program**
- All **string literals** have a `'static` lifetime — they are stored in the program binary and never dropped:
  ```rust
  let s: &'static str = "Hello, world!";
  ```
- A `'static` reference can be **coerced** to a shorter lifetime when passed to a function with a more restrictive annotation, though this is rare
- `'static` can also be used as a **trait bound** to ensure a generic type contains no non-static references:
  ```rust
  fn keep_forever<T: 'static>(item: T) { ... }
  ```
- This guarantees the receiver can hold onto the value indefinitely without it becoming invalid