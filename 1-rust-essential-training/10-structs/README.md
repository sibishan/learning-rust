# Structs

## 1. Defining Structs

- A **struct** (structure) packages related values of different types together — like a tuple but with **named fields**
- Define a struct with the `struct` keyword — use **PascalCase** for the name:
  ```rust
  struct Shuttle {
      name: String,
      crew_size: u8,
      propellant: f64,
  }
  ```
- **Instantiate** a struct by providing concrete values for each field:
  ```rust
  let vehicle = Shuttle {
      name: String::from("Endeavor"),
      crew_size: 7,
      propellant: 835_958.0,
  };
  ```
- Access fields using **dot notation**:
  ```rust
  println!("{}", vehicle.name);
  ```
- To modify fields, declare the instance as `mut`:
  ```rust
  let mut vehicle = Shuttle { ... };
  vehicle.name = String::from("Atlantis");
  ```
- Use `{:?}` (debug formatting) to print a struct — add `#[derive(Debug)]` above the definition:
  ```rust
  #[derive(Debug)]
  struct Shuttle { ... }
  println!("{:?}", vehicle);
  ```
- Stack-only fields live on the stack; heap-based fields (like `String`) store their pointer on the stack while data lives on the heap — the struct owns the string, so both are dropped when the struct goes out of scope

---

## 2. Struct Update Syntax

- Create a new struct instance that copies fields from an existing one using `..`:
  ```rust
  let vehicle2 = Shuttle {
      name: String::from("Discovery"),
      ..vehicle // copy remaining fields from vehicle
  };
  ```
- Any field not explicitly set takes the value from the existing instance
- The two instances are **independent** — later changes to one do not affect the other
- **Ownership caveat:** if a heap-based field (like `String`) is copied via `..`, ownership is **moved** — the original instance can no longer use that field:
  ```rust
  let vehicle2 = Shuttle { ..vehicle }; // moves vehicle.name
  // vehicle.name is now invalid
  ```
- Fix by **cloning** — add `#[derive(Clone)]` to the struct definition:
  ```rust
  #[derive(Debug, Clone)]
  struct Shuttle { ... }

  let vehicle2 = vehicle.clone();
  ```

---

## 3. Struct Methods

- **Methods** are functions defined in the context of a struct using an `impl` block:
  ```rust
  impl Shuttle {
      fn get_name(&self) -> &str {
          &self.name
      }
  }
  ```
- The first parameter is always `self` (a reference to the struct instance):
  - `&self` — immutable borrow (read-only access)
  - `&mut self` — mutable borrow (can modify fields)
- Call a method using dot notation on an instance:
  ```rust
  let name = vehicle.get_name();
  ```
- Example of a mutating method:
  ```rust
  fn add_fuel(&mut self, gallons: f64) {
      self.propellant += gallons;
  }

  vehicle.add_fuel(1000.0);
  ```
- Encapsulating modifications in methods allows you to add validation logic (e.g. prevent overfilling)

---

## 4. Associated Functions

- **Associated functions** are defined in `impl` blocks but do **not** take `self` as a parameter — they belong to the type, not an instance
- Commonly used as **constructors** to build new instances with default values:
  ```rust
  impl Shuttle {
      fn new(name: &str) -> Shuttle {
          Shuttle {
              name: String::from(name),
              crew_size: 7,
              propellant: 0.0,
          }
      }
  }
  ```
- Call using the **double colon path operator** on the type name (not an instance):
  ```rust
  let vehicle = Shuttle::new("Endeavor");
  let vehicle2 = Shuttle::new("Discovery");
  ```
- This pattern is the same as `String::from(...)` — `from` is an associated function on `String`

---

## 5. Tuple Structs

- **Tuple structs** are like tuples with a named type — fields have no names, just types:
  ```rust
  struct Color(u8, u8, u8); // red, green, blue
  struct Point(u8, u8, u8); // x, y, z
  ```
- Instantiate like a tuple:
  ```rust
  let red = Color(255, 0, 0);
  let coord = Point(10, 20, 30);
  ```
- Access fields by index using dot notation:
  ```rust
  println!("{}", red.0); // 255
  ```
- Although `Color` and `Point` both contain three `u8` values, they are **distinct types** — you cannot pass a `Color` where a `Point` is expected:
  ```rust
  fn get_y(p: Point) -> u8 { p.1 }
  get_y(red); // ERROR — expected Point, got Color
  ```
- Use tuple structs when naming every field would be verbose but you still want a uniquely named type