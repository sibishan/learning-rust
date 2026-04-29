# Traits

## 1. Implementing Traits

- A **trait** defines a collection of method signatures representing a set of behaviours — any type that implements the trait must provide those methods
- Similar to **interfaces** in C++ or Java
- Define a trait with the `trait` keyword:
  ```rust
  trait Description {
      fn describe(&self) -> String;
  }
  ```
- Implement a trait for a specific struct using `impl <Trait> for <Type>`:
  ```rust
  impl Description for Satellite {
      fn describe(&self) -> String {
          format!("{} is flying at {} miles per second", self.name, self.velocity)
      }
  }
  ```
- Once implemented, the trait's methods are available on instances of that type:
  ```rust
  println!("{}", hubble.describe());
  ```
- The value of traits becomes clear with generics — traits let you set bounds on what types are acceptable

---

## 2. Default Trait Implementation

- Traits can include a **default implementation** for methods — used when a type doesn't provide its own:
  ```rust
  trait Description {
      fn describe(&self) -> String {
          String::from("An object flying through space")
      }
  }
  ```
- A type can still override the default by implementing its own version in its `impl` block
- If a type implements the trait but provides no method body, the default is used:
  ```rust
  impl Description for Satellite {} // uses default describe()
  ```
- Useful for large traits where only some types need custom behaviour

---

## 3. Derive Traits

- The Rust compiler can **automatically generate** implementations for a handful of common traits using the `#[derive]` attribute:
  ```rust
  #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
  struct Satellite {
      name: String,
      velocity: f64,
  }
  ```
- Common derivable traits:

| Trait         | Capability                                      |
|---------------|-------------------------------------------------|
| `Debug`       | Format with `{:?}` for debug output             |
| `PartialEq`   | Compare with `==` and `!=`                      |
| `PartialOrd`  | Compare with `>`, `<`, `>=`, `<=`               |
| `Clone`       | Explicitly duplicate heap data                  |
| `Copy`        | Implicitly copy stack data                      |
| `Default`     | Create an empty/zero instance of the type       |

- Derived `PartialEq` considers two instances equal only if **all fields are equal**
- Derived `PartialOrd` compares fields **in order of definition** until one differs — may not always be meaningful for your type
- Derive multiple traits by separating them with commas

---

## 4. Trait Bounds

- **Trait bounds** restrict a generic type to only types that implement specific traits:
  ```rust
  use std::fmt;

  fn print_type<T: fmt::Display>(item: T) {
      println!("{}", item);
  }
  ```
- This guarantees that whatever type `T` is, it will have the required methods (`Display` here) for the function to work
- Passing a type that doesn't meet the bound causes a **compiler error**
- Use `fmt::Debug` and `{:?}` if the type implements `Debug` but not `Display`:
  ```rust
  fn print_type<T: fmt::Debug>(item: T) {
      println!("{:?}", item);
  }
  ```

---

## 5. Multiple Trait Bounds

- Combine multiple trait bounds with `+`:
  ```rust
  fn compare_and_print<T: fmt::Display + PartialEq, U: fmt::Display + PartialEq>(a: T, b: U) {
      ...
  }
  ```
- For long signatures, use a **`where` clause** for readability:
  ```rust
  fn compare_and_print<T, U>(a: T, b: U)
  where
      T: fmt::Display + PartialEq + From<U>,
      U: fmt::Display + Copy,
  {
      ...
  }
  ```
- Common additional bounds needed:
  - `From<U>` — allows converting type `U` into type `T` using the `.from()` method
  - `Copy` — required if a variable is used after being passed to a function (prevents it from being moved)
- Both syntaxes compile identically — `where` is preferred when bounds become long

---

## 6. Return Types with Implemented Traits

- Use `impl <Trait>` as a return type to return any type that implements that trait — the exact type doesn't need to be named:
  ```rust
  use std::fmt;

  fn get_displayable() -> impl fmt::Display {
      13
  }
  ```
- The caller only knows the return value implements the given trait — they can use any methods that trait provides
- The returned type can be changed to any other type that also implements the trait:
  ```rust
  fn get_displayable() -> impl fmt::Display {
      "hello" // also valid — &str implements Display
  }
  ```
- **Limitation:** the function must always return the **same concrete type** — you cannot return different types conditionally:
  ```rust
  // DOES NOT COMPILE
  fn get_displayable(choice: bool) -> impl fmt::Display {
      if choice { 13 }        // i32
      else { "thirteen" }     // &str — different type, not allowed
  }
  ```
- Returning different types based on runtime conditions requires **dynamic dispatch** (an advanced topic)