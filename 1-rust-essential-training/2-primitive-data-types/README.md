# Primitive Data Types & Operations

## 1. Declaring Variables

- Declare variables with the `let` keyword:
  ```rust
  let x = 10;
  ```
- By default, variables are **immutable** — you cannot reassign them after declaration
- To make a variable mutable, use the `mut` keyword:
  ```rust
  let mut x = 10;
  x = 20; // valid
  ```
- Variable names can contain letters, digits, and underscores — but **cannot start with a digit**
- Names are **case-sensitive** and cannot be reserved keywords (e.g. `let`, `mut`)
- Convention: use **snake_case** for variable names (e.g. `my_variable`)
- Use `{}` as a placeholder in `println!` to display a variable's value:
  ```rust
  println!("X is {}", x);
  ```

---

## 2. Integer Data Types

- Integers are characterized by **bit size** and **signedness**
- **Unsigned** (`u`): positive values only — `u8`, `u16`, `u32`, `u64`, `u128`
- **Signed** (`i`): positive and negative — `i8`, `i16`, `i32`, `i64`, `i128`
- Default integer type is **`i32`** (signed 32-bit)
- Specify a type explicitly with a colon:
  ```rust
  let x: u8 = 255;
  ```
- Common ranges:
  - `u8`: 0 to 255
  - `i8`: -128 to 127
  - `i32`: ~±2 billion
- Rust will **panic at runtime** if an integer overflows its valid range
- The compiler catches obvious errors (e.g. storing a negative value in an unsigned type)

---

## 3. Floating-Point Data Types

- Use floats to store numbers with decimal places
- Two types: `f32` (32-bit, ~6–9 significant digits) and `f64` (64-bit, ~15–17 significant digits)
- Default float type is **`f64`**
- Declare by including a decimal point:
  ```rust
  let x = 3.14;       // f64 by default
  let y: f32 = 3.14;  // explicit f32
  ```
- Floats have **limited precision** — stored values may not be exactly what you wrote
- General rule: use `f64` unless memory is constrained (e.g. embedded systems)

---

## 4. Arithmetic Operations

| Operator | Operation      | Symbol |
|----------|----------------|--------|
| Add      | Addition       | `+`    |
| Subtract | Subtraction    | `-`    |
| Multiply | Multiplication | `*`    |
| Divide   | Division       | `/`    |
| Modulo   | Remainder      | `%`    |

- Integer division **truncates** the remainder (e.g. `10 / 3 = 3`)
- Use `%` to get the remainder (e.g. `10 % 3 = 1`)
- Float division preserves the decimal (e.g. `10.0 / 3.0 = 3.333...`)
- **Cannot mix types** in arithmetic — use casting with `as`:
  ```rust
  let result = a as f64 / b;
  ```
- Casting can **lose information** (e.g. `3.9 as i32 = 3`, truncation not rounding)
- **Operator precedence**: `*`, `/`, `%` before `+`, `-` — use parentheses to control order

---

## 5. Formatting Print Statements

- Basic display: `println!("Value: {}", x);`
- Decimal places: `{:.3}` → 3 decimal places
- Total width: `{:8.3}` → 8 characters wide, 3 decimal places
- Leading zeros: `{:08.3}` → pad with zeros instead of spaces
- Multiple variables:
  ```rust
  println!("c is {}, a is {}", c, a);
  ```
- Positional parameters (reuse variables):
  ```rust
  println!("{0} and again {0}, then {1}", c, a);
  ```
- Newline character: `\n` inside a string inserts a line break
- `print!` (no `ln`) does **not** append a newline at the end
- Rust 1.58+: capture variables directly by name:
  ```rust
  println!("x is {x}");
  ```

---

## 6. Bitwise Operations

- Define values in binary using `0b` prefix:
  ```rust
  let x: u8 = 0b1111_0101;
  ```
- Display as binary: `{:08b}` in the format string

| Operator    | Symbol | Description                              |
|-------------|--------|------------------------------------------|
| NOT         | `!`    | Inverts all bits                         |
| AND         | `&`    | 1 if both bits are 1                     |
| OR          | `\|`   | 1 if either bit is 1                     |
| XOR         | `^`    | 1 if bits are different                  |
| Left Shift  | `<<`   | Shifts bits left, fills right with zeros |
| Right Shift | `>>`   | Shifts bits right, fills left with zeros |

- **AND** is used to **clear** a bit or **check** its value
- **OR** is used to **set** a bit to 1
- **XOR** is used to **toggle** or **compare** bit patterns
- Bits shifted off the edge are **lost**

---

## 7. Boolean Data Type and Operations

- A `bool` holds either `true` or `false`
- Corresponds to integer `1` (true) and `0` (false)
- Logical operators:

| Operator | Symbol | Description                   |
|----------|--------|-------------------------------|
| NOT      | `!`    | Flips the value               |
| AND      | `&`    | True if both are true         |
| OR       | `\|`   | True if either is true        |
| XOR      | `^`    | True if values are different  |

- **Short-circuit operators** (more efficient):
  - `&&` — skips right side if left is `false`
  - `||` — skips right side if left is `true`
- Use short-circuit operators to avoid unnecessary evaluation

---

## 8. Comparison Operations

| Operator | Meaning               |
|----------|-----------------------|
| `==`     | Equal to              |
| `!=`     | Not equal to          |
| `>`      | Greater than          |
| `>=`     | Greater than or equal |
| `<`      | Less than             |
| `<=`     | Less than or equal    |

- All comparisons return a **Boolean** result (`true` or `false`)
- Can compare any data type that supports comparison traits (numbers, booleans, chars, etc.)
- **Cannot compare values of different types** — will cause a compiler error

---

## 9. Char Data Type

- Represents a **single character**, declared with **single quotes**:
  ```rust
  let letter = 'a';
  let num = '1';
  ```
- Rust `char` is a **Unicode scalar value** — uses **4 bytes** (unlike C/C++ which uses 1 byte)
- Can represent symbols, emojis, and non-Latin characters
- Specify by Unicode hex code:
  ```rust
  let symbol = '\u{261D}'; // ☝
  ```
- A `char` holding `'1'` is **not** the same as the integer `1` — arithmetic does not apply to chars
- Some terminals may not render exotic Unicode characters correctly