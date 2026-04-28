# References, Borrowing & Slices

## 1. Borrowing References

- **Borrowing** allows a function to access data without taking ownership of it
- Use the **borrow operator** `&` to create a reference:
  ```rust
  fn process_fuel(propellant: &String) -> usize {
      propellant.len()
  }

  let length = process_fuel(&rocket_fuel);
  ```
- The original variable (`rocket_fuel`) **retains ownership** — the reference just points to the data
- When the reference goes out of scope, the heap data is **not dropped** because it was never owned by the reference
- Borrowing avoids the cumbersome pattern of transferring ownership in and then returning it back out just to use a value
- When working with heap-stored types, passing by reference is far more common than passing by value

---

## 2. Mutable References

- By default, borrowed references are **immutable** — you cannot modify the data through them
- To modify borrowed data, explicitly declare a **mutable reference** with `&mut`:
  ```rust
  fn process_fuel(propellant: &mut String) {
      propellant.push_str(" is highly flammable");
  }

  process_fuel(&mut rocket_fuel); // pass a mutable reference
  ```
- The original variable must also be declared `mut`
- **Key restriction:** once a mutable reference exists, **no other references** (mutable or immutable) can exist to the same variable in that scope
- This prevents **data races** — a common concurrency bug where multiple references simultaneously access and modify the same data
- Summary of reference rules:
  - Any number of **immutable** references — allowed simultaneously
  - Exactly **one mutable** reference — and no other references at the same time

---

## 3. Dangling References

- A **dangling reference** occurs when a reference points to memory that has already been freed:
  ```rust
  fn produce_fuel() -> &String {
      let new_fuel = String::from("RP-1");
      &new_fuel // ERROR — new_fuel is dropped when function ends
  }
  ```
- `new_fuel` goes out of scope at the end of the function, dropping the string — the reference would point to freed memory
- Rust **guarantees** this can never happen — the compiler checks that data will not go out of scope before any reference to it does
- The fix is to **return the value itself** (transferring ownership) instead of a reference to it:
  ```rust
  fn produce_fuel() -> String {
      let new_fuel = String::from("RP-1");
      new_fuel // ownership transferred to caller — no dangling reference
  }
  ```

---

## 4. Slices

- A **slice** references a contiguous subset of elements in a collection without taking ownership
- String slice type is written as `&str` — this is also what string literals are
- Create a string slice using a range index in square brackets:
  ```rust
  let message = String::from("Greetings from Earth");
  let last_word = &message[15..20]; // "Earth"
  ```
- Omit the end index to slice to the end of the collection:
  ```rust
  let last_word = &message[15..]; // "Earth"
  ```
- The slice stores a **pointer** to the start of the subset and the **length** — it does not own any heap data
- **Important:** slice indices are in **bytes**, not characters — indexing in the middle of a multi-byte UTF-8 character will cause a **runtime panic**
- Arrays can also be sliced:
  ```rust
  let planets = [1, 2, 3, 4, 5, 6, 7, 8];
  let inner: &[i32] = &planets[0..4]; // [1, 2, 3, 4]
  ```
- Out-of-bounds slice indices are not always caught at compile time — they will panic at runtime

---

## 5. Slices as Function Parameters

- Slices are commonly used as function input and output when working with strings:
  ```rust
  fn get_first_word(s: &str) -> &str {
      // iterate bytes, return slice up to first space
  }
  ```
- **String reference (`&String`) vs string slice (`&str`):**
  - `&String` points to a String on the stack, which in turn owns heap data — includes pointer, length, and capacity
  - `&str` points directly to heap data with just a pointer and length — no capacity (slices never own heap data)
  - Rust allows using a `&String` where `&str` is expected (**Deref coercion**) — the string is treated as a full-length slice
  - The reverse does **not** work — `&str` cannot be used in place of `&String`
- **Best practice:** use `&str` for string function parameters — it's more flexible and works with both string references and slices:
  ```rust
  fn get_first_word(s: &str) -> &str { ... }

  get_first_word(&my_string); // works via Deref coercion
  get_first_word(&my_string[5..]); // also works
  ```