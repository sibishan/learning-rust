# Vectors & HashMaps

## 1. Vectors

- A **vector** (`Vec<T>`) holds a collection of same-type elements in sequential order — like an array, but **dynamically sized**
- Stored on the **heap** — ownership and borrowing rules apply the same as `String`
- Create an empty vector (must specify type, and declare `mut` to modify):
  ```rust
  let mut astronauts: Vec<String> = Vec::new();
  ```
- Create a pre-populated vector using the `vec!` macro:
  ```rust
  let countdown = vec![5, 4, 3, 2, 1];
  ```
- Add elements to the end with `.push()`:
  ```rust
  astronauts.push(String::from("Glenn"));
  ```
- Remove and return the last element with `.pop()` — returns an `Option<T>`:
  ```rust
  let last = astronauts.pop(); // Some("Glenn") or None
  ```
- Access elements by index using `[]` — panics at runtime if out of bounds:
  ```rust
  let third = &astronauts[2];
  ```
- Access elements safely with `.get()` — returns `Option<&T>`, never panics:
  ```rust
  let third = astronauts.get(2); // Some(&"Glenn") or None
  ```
- Use borrow (`&`) when accessing elements — the vector retains ownership
- Print with debug formatting `{:?}` since vectors don't implement `Display`
- The compiler **cannot** check vector index bounds at compile time (unlike arrays) — prefer `.get()` for safety

---

## 2. HashMaps

- A **HashMap** stores data as **key-value pairs** — look up values by providing a key:
  ```rust
  use std::collections::HashMap;
  ```
- Not in the prelude — must be imported manually
- Create a new HashMap and insert entries:
  ```rust
  let mut missions_flown: HashMap<&str, i32> = HashMap::new();
  missions_flown.insert("Hadfield", 3);
  missions_flown.insert("Hurley", 3);
  missions_flown.insert("Barron", 0);
  ```
- The compiler can **infer key/value types** from the inserted data
- All keys must be the same type; all values must be the same type
- **No duplicate keys** — each key maps to exactly one value
- The order keys are stored is **not guaranteed** — do not rely on insertion order
- Retrieve a value with `.get()` — returns `Option<&V>`:
  ```rust
  let count = missions_flown.get("Barron"); // Some(&0) or None
  ```

**Three ways to update a HashMap:**

- **Overwrite** an existing entry — insert with the same key:
  ```rust
  missions_flown.insert("Barron", 1); // replaces old value
  ```
- **Insert only if the key doesn't exist** using `.entry().or_insert()`:
  ```rust
  missions_flown.entry("Stone").or_insert(2); // adds "Stone" if missing
  ```
- **Update based on the existing value** — `.or_insert()` returns a mutable reference:
  ```rust
  let count = missions_flown.entry("Barron").or_insert(0);
  *count += 1; // dereference to modify the value in place
  ```