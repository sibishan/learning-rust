# Generics & Box

## 1. Generic Struct Definitions

- **Generics** are abstract stand-ins for concrete data types — they eliminate duplicate code when a struct needs to work with multiple types
- Define a generic type using angle brackets after the struct name — conventionally use single capital letters starting with `T`:
  ```rust
  struct Rectangle<T> {
      width: T,
      height: T,
  }
  ```
- Instantiate with any concrete type — the compiler infers and substitutes `T`:
  ```rust
  let rect = Rectangle { width: 1u8, height: 3u8 };
  ```
- If two fields need **different** generic types, add more variables:
  ```rust
  struct Rectangle<T, U> {
      width: T,
      height: U,
  }
  ```
- **Generics are zero-cost** — the compiler uses **monomorphization** to replace generic placeholders with concrete types at compile time, so there is no runtime overhead

---

## 2. Generic Method Definitions

- When implementing methods for a generic struct, include the generic variables after `impl` and after the struct name:
  ```rust
  impl<T, U> Rectangle<T, U> {
      fn get_width(&self) -> &T {
          &self.width
      }
  }
  ```
- Return a **reference** (`&T`) rather than the value itself — since `T` is unknown, it could be heap-based, and returning by value could unexpectedly transfer ownership
- To define methods that apply **only to a specific concrete type**, omit the generics after `impl` and specify the concrete types after the struct name:
  ```rust
  impl Rectangle<u8, u8> {
      fn get_perimeter(&self) -> u8 {
          2 * self.width + 2 * self.height
      }
  }
  ```
- Calling a type-specific method on a non-matching instance (e.g. `Rectangle<u8, u16>`) causes a **compiler error**

---

## 3. Generic Function Definitions

- Generics can also be used in standalone functions:
  ```rust
  fn get_biggest<T>(a: T, b: T) -> T {
      if a > b { a } else { b }
  }
  ```
- If the function uses comparison operators (like `>`), the generic type must be **bounded** to types that support comparison — add `: PartialOrd` after the type variable:
  ```rust
  fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
      if a > b { a } else { b }
  }
  ```
- `PartialOrd` is a **trait** (included in the prelude) that defines the ability to compare two values — only types implementing it can be used with comparison operators
- With bounds in place, the function works for any comparable type — integers, floats, etc.:
  ```rust
  get_biggest(1, 2);       // 2
  get_biggest(1.5, 3.0);   // 3.0
  ```

---

## 4. Box Data Type

- `Box<T>` stores data of type `T` on the **heap**, even if `T` would normally live on the stack
- A box is a **smart pointer** — it owns the heap data and automatically drops and deallocates it when it goes out of scope
- Create a box using `Box::new()`:
  ```rust
  let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
  ```
- Moving a value into a box **transfers ownership** — the original variable is no longer valid
- The box pointer lives on the **stack** (8 bytes); the actual data lives on the **heap**
- Use the **dereference operator** `*` to access the heap data through the pointer:
  ```rust
  let size = std::mem::size_of_val(&*boxed_vehicle); // size of heap data
  ```
- Move data **back to the stack** by dereferencing:
  ```rust
  let unboxed_vehicle: Shuttle = *boxed_vehicle;
  ```
- Common use cases for `Box`:
  - Storing **recursive data types** whose total size cannot be known at compile time (e.g. a struct that contains another instance of itself)
  - **Transferring ownership** of large stack data without copying — moving a pointer is cheaper than copying the whole structure