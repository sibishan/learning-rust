# Functions

## 1. Function Parameters

- Declare functions with the `fn` keyword, a name, parentheses, and curly braces:
  ```rust
  fn say_hello() {
      println!("Hello!");
  }
  ```
- Call a function by its name followed by parentheses:
  ```rust
  say_hello();
  ```
- Rust does **not** require functions to be defined before they are called
- **Parameters** are variables defined in the function signature to accept input:
  ```rust
  fn say_a_number(number: i32) {
      println!("{}", number);
  }
  ```
- Rust requires a **data type annotation** for every parameter
- The value passed in when calling the function is called an **argument**; the name used inside the function is the **parameter** (these terms are often used interchangeably)
- Multiple parameters are separated by commas and can have different types:
  ```rust
  fn say_the_sum(a: u8, b: u8) {
      let sum = a + b;
      println!("{}", sum);
  }
  ```
- The compiler is smart enough to **infer the type of a variable** based on how it's used as a function argument — if you don't annotate a variable but pass it to a function expecting `u8`, the compiler will treat it as `u8`

---

## 2. Statements vs. Expressions

- A **statement** performs an action but **does not return a value** — it ends with a semicolon:
  ```rust
  x = 1; // statement — assigns a value, returns nothing
  ```
- An **expression** evaluates to produce a resulting value — it has **no semicolon**:
  ```rust
  1 + 2  // expression — evaluates to 3
  ```
- Adding a semicolon to an expression **turns it into a statement**, discarding its result
- Expressions commonly appear as part of statements:
  ```rust
  let sum = a + b; // "a + b" is the expression, the whole line is a statement
  ```
- This distinction matters for **function return values** — the last line of a function without a semicolon is returned as the output

---

## 3. Function Return Values

- Specify the return type in the function signature using `->`:
  ```rust
  fn square(x: i32) -> i32 {
      x * x  // expression — returned because no semicolon
  }
  ```
- The **last expression** in a function is automatically returned — no `return` keyword needed
- Use the `return` keyword to return early (must add a semicolon):
  ```rust
  fn square(x: i32) -> i32 {
      return x * x; // explicit early return
  }
  ```
- To return **multiple values**, use a tuple:
  ```rust
  fn square(x: i32) -> (i32, i32) {
      (x, x * x)
  }
  ```
- Use debug formatting `{:?}` to print a tuple:
  ```rust
  println!("{:?}", result);
  ```
- Functions with no return value implicitly return the **unit type** `()` (an empty tuple) — you don't need to write this explicitly