# Enums & Pattern Matching

## 1. Define Enums

- An **enum** (enumeration) is a data type with a finite set of possible **variants**
- Define with the `enum` keyword — use PascalCase:
  ```rust
  enum Shape {
      Circle,
      Rectangle,
      Triangle,
  }
  ```
- Instantiate a variant using the double colon path operator:
  ```rust
  let my_shape = Shape::Rectangle;
  ```
- Variants can also **store data** — each variant can have different types and amounts:
  ```rust
  enum Shape {
      Circle(f64),           // radius
      Rectangle(f64, f64),   // width, height
      Triangle(f64, f64, f64), // three sides
  }

  let my_shape = Shape::Rectangle(1.2, 3.4);
  ```
- Use `#[derive(Debug)]` and `{:?}` to print enums
- Enums can hold other enums, structs, tuples, or any other data type within their variants

---

## 2. Match Operator

- The `match` operator compares a value against a series of **patterns** and executes the first one that matches — similar to a switch statement:
  ```rust
  match my_shape {
      Shape::Circle(r) => println!("Circle with radius {}", r),
      Shape::Rectangle(w, h) => println!("{}x{} Rectangle", w, h),
      Shape::Triangle(a, b, c) => println!("Triangle: {}, {}, {}", a, b, c),
  }
  ```
- Patterns for enum variants **capture the stored data** as named variables
- `match` evaluates arms **top to bottom** and executes the first match
- `match` can also **return a value** — all arms must return the same type:
  ```rust
  let result = match my_shape {
      Shape::Circle(r) => r * 2.0,
      _ => 0.0,
  };
  ```

---

## 3. Match with Default Placeholder

- Every `match` expression **must cover all possible cases** — the compiler enforces this
- Use `_` as a **wildcard/default** pattern to catch any value not explicitly handled:
  ```rust
  let result = match my_number {
      0 => "zero",
      1 => "one",
      2 => "two",
      _ => "something else", // handles all remaining cases
  };
  ```
- The wildcard must be placed **last** — any arm after it would be unreachable
- To execute multiple statements in a match arm, use curly braces:
  ```rust
  _ => {
      println!("Did not match");
      "something else"
  }
  ```
- Leaving any possible value unhandled causes a **compiler error** — even if that value seems unreachable from the current code

---

## 4. Enum Methods

- Like structs, enums can have methods defined in an `impl` block:
  ```rust
  impl Shape {
      fn get_perimeter(&self) -> f64 {
          match self {
              Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
              Shape::Rectangle(w, h) => 2.0 * w + 2.0 * h,
              Shape::Triangle(a, b, c) => a + b + c,
          }
      }
  }
  ```
- Call on an instance using dot notation:
  ```rust
  println!("{}", my_shape.get_perimeter());
  ```
- `match` expressions are commonly used inside enum methods to branch on the variant

---

## 5. Option\<T\> Enum

- Rust has **no null value** — instead it uses the `Option<T>` enum to represent the concept of something or nothing:
  ```rust
  enum Option<T> {
      Some(T), // has a value
      None,    // no value
  }
  ```
- `Option` is included in the **prelude** — no import needed
- Use `Some(value)` to wrap a value, `None` for absence:
  ```rust
  let x: Option<i32> = Some(5);
  let y: Option<i32> = None;
  ```
- The `.get()` method on arrays/slices returns an `Option` instead of panicking on out-of-bounds:
  ```rust
  let numbers = [1, 2, 3, 4, 1];
  let num = numbers.get(4); // Some(&1)
  let oob = numbers.get(5); // None
  ```
- You **cannot use the inner value directly** — it must be extracted from the `Option` first
- Extract safely with `.unwrap_or(default)` — returns the value or a fallback if `None`:
  ```rust
  let val = numbers.get(5).unwrap_or(&0); // returns &0 if None
  ```
- Avoid `.unwrap()` alone — it panics if the value is `None`

---

## 6. Matching Option\<T\>

- Use `match` to handle both `Some` and `None` cases explicitly:
  ```rust
  let result = match number {
      Some(n) => n + 1,
      None => 0,
  };
  ```
- This is the idiomatic Rust way to safely work with `Option` values — the compiler enforces that both cases are handled
- The `Some(n)` arm captures the inner value as `n` to use in the expression

---

## 7. If-Let Syntax

- When you only care about **one specific pattern** and want to ignore all others, `if let` is a cleaner alternative to `match`:
  ```rust
  // match version
  match number {
      Some(13) => println!("13"),
      _ => (),
  }

  // if let version
  if let Some(13) = number {
      println!("13");
  }
  ```
- `if let` is **syntactic sugar** for a match that only acts on one arm and does nothing for all others
- Useful when handling a single `Some` value, a specific enum variant, or any single pattern
- An `else` block can be added to handle the non-matching case:
  ```rust
  if let Some(n) = number {
      println!("Got {}", n);
  } else {
      println!("Got nothing");
  }
  ```