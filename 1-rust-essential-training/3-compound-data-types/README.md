# Compound Data Types

## 1. Arrays

- Arrays hold a **fixed-length collection of elements of the same data type**
- Elements are stored sequentially in memory
- Arrays use **zero-based indexing** (first element is index `0`)
- Declare with a comma-separated list in square brackets:
  ```rust
  let letters = ['A', 'B', 'C'];
  ```
- Access elements with square bracket indexing:
  ```rust
  let first = letters[0]; // 'A'
  ```
- To modify elements, declare the array as mutable:
  ```rust
  let mut letters = ['A', 'B', 'C'];
  letters[0] = 'X';
  ```
- Declare an array with a known type and length but no initial values:
  ```rust
  let numbers: [i32; 5];
  ```
- Initialize all elements to the same value using a **repeat expression**:
  ```rust
  let numbers = [0; 5]; // [0, 0, 0, 0, 0]
  ```
- Arrays **must be indexed using `usize`** (not a regular integer type)
- Accessing an out-of-bounds index causes a **runtime panic** — Rust prevents invalid memory access (unlike C/C++)

---

## 2. Multidimensional Arrays

- Create a 2D array by nesting arrays inside an outer array:
  ```rust
  let numbers = [
      [1, 2, 3],
      [4, 5, 6],
  ];
  ```
- Index with two values — first selects the sub-array, second selects the element:
  ```rust
  let val = numbers[0][1]; // 2 (row 0, spot 1)
  ```
- All inner arrays **must have the same type and length** — mismatched sizes cause a compiler error
- For a 3D array, declare the type by nesting:
  ```rust
  let garage = [[[0i32; 100]; 20]; 5]; // 5 floors x 20 rows x 100 spots
  ```
- Think of it as: 1D = row of cars, 2D = parking lot, 3D = multi-story garage

---

## 3. Tuples

- Tuples group **related items of different data types** into a single compound value
- Fixed length, stored contiguously in memory
- Declare with parentheses:
  ```rust
  let stuff = (10, 3.14, 'x');
  ```
- Specify types explicitly if needed:
  ```rust
  let stuff: (u8, f32, char) = (10, 3.14, 'x');
  ```
- Access elements using **dot notation** with a zero-based index:
  ```rust
  let first = stuff.0;  // 10
  let second = stuff.1; // 3.14
  ```
- To modify elements, declare the tuple as mutable:
  ```rust
  let mut stuff = (10, 3.14, 'x');
  stuff.0 += 3; // stuff.0 is now 13
  ```
- **Destructuring** — break a tuple into individual variables:
  ```rust
  let (a, b, c) = stuff;
  // a = 10, b = 3.14, c = 'x'
  ```