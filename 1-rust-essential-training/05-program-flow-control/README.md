# Program Flow Control

## 1. Conditional Execution

- Use `if` to conditionally execute a block of code based on a boolean condition:
  ```rust
  if x == 3 {
      println!("x is three");
  }
  ```
- The condition **must evaluate to a boolean** — Rust does not allow numeric values as conditions (unlike C++)
- The condition can include comparison operators, math, and logical operators:
  ```rust
  if x + 1 == 3 { ... }
  ```

---

## 2. Multiple Conditions

- Add an `else` block to handle the false case:
  ```rust
  if x > y {
      println!("x is greater");
  } else {
      println!("x is not greater");
  }
  ```
- Use `else if` to chain multiple conditions:
  ```rust
  if x > y {
      println!("x is greater");
  } else if x < y {
      println!("x is less");
  } else {
      println!("x equals y");
  }
  ```
- Nested `if/else` blocks and `else if` chains accomplish the same thing — `else if` is generally cleaner and preferred

---

## 3. Conditional Assignment

- Rust's `if` is an **expression**, meaning it returns a value and can be used in assignments:
  ```rust
  let x = if make_x_odd { 1 } else { 2 };
  ```
- This is a compact alternative to declaring a variable and assigning it inside an `if/else` block
- **All branches must return the same data type** — mixing types (e.g. integer in `if`, float in `else`) causes a compiler error
- The Rust compiler checks **every possible execution path** to ensure a variable is always initialized before use

---

## 4. Loops

- `loop` creates an **infinite loop** that repeats until explicitly stopped:
  ```rust
  loop {
      count += 1;
      println!("{}", count);
  }
  ```
- Use `break` to exit the loop:
  ```rust
  if count == 10 {
      break;
  }
  ```
- `loop` can **return a value** via `break`:
  ```rust
  let result = loop {
      count += 1;
      if count == 10 {
          break count * 10;
      }
  };
  ```
- The `let` statement wrapping the loop needs a semicolon at the end
- Use `loop` when you need to repeat indefinitely or need to return a value on exit

---

## 5. While Loops

- `while` repeats a block **as long as a condition is true**:
  ```rust
  while count < 10 {
      count += 1;
      println!("{}", count);
  }
  ```
- The condition is checked **before each iteration** — if false from the start, the loop never runs
- `while` can use `break` to exit early but **cannot return a value** like `loop`
- Using a `while` loop with a counter to index an array is possible but risky — prefer `for` loops for iterating collections
- To create an infinite loop with `while`: `while true { ... }` — but prefer `loop` for this

---

## 6. For Loops

- `for` iterates over each item in a collection:
  ```rust
  for item in message {
      println!("{}", item);
  }
  ```
- Internally converts the collection into an **iterator** and calls `next` automatically
- For Rust versions before 1.53, call `.iter()` explicitly:
  ```rust
  for item in message.iter() { ... }
  ```
- Use `.enumerate()` to get the index alongside each item:
  ```rust
  for (i, item) in message.iter().enumerate() {
      println!("{}: {}", i, item);
  }
  ```
- Use `break` to exit early; when comparing against array elements, use `&` to dereference:
  ```rust
  if item == &'e' { break; }
  ```
- Iterate over a **range** of numbers (start inclusive, end exclusive):
  ```rust
  for i in 0..5 {
      println!("{}", i); // prints 0 to 4
  }
  ```
- Use `for` when iterating over collections or repeating a known number of times

---

## 7. Nested Loops

- Loops can be nested inside each other — common for processing multidimensional arrays:
  ```rust
  for row in &matrix {
      for num in row {
          print!("{}\t", num);
      }
      println!();
  }
  ```
- `\t` inserts a tab character; `print!` (no `ln`) avoids automatic newlines
- To **modify** values while iterating, declare the array as `mut` and use `.iter_mut()`:
  ```rust
  for row in matrix.iter_mut() {
      for num in row.iter_mut() {
          *num += 10; // dereference with * to modify the value
      }
  }
  ```
- The `*` operator dereferences the mutable reference returned by the iterator

---

## Loop Selection Guide

| Use case                                      | Loop type |
|-----------------------------------------------|-----------|
| Repeat forever / need a return value on exit  | `loop`    |
| Repeat while a condition is true              | `while`   |
| Iterate over a collection or range            | `for`     |