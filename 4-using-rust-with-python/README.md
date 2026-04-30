# Using Rust with Python

---

## 1. Rust and Python — When to Use Each

| Python                              | Rust                                      |
|-------------------------------------|-------------------------------------------|
| Readability and productivity        | True threads and concurrency              |
| Huge ecosystem of libraries         | Machine-level performance                 |
| Great for scripts, web apps, CLIs   | Memory safety and compiler-enforced checks|
| Many competing packaging solutions  | One unified package manager: Cargo        |
| No true threads (GIL limitation)    | Low energy consumption                    |

- **Intersection:** Use Python for rapid development and rich libraries; use Rust for performance-critical, memory-intensive, or concurrent workloads — then expose the Rust code to Python
- **Generative AI angle:** Rust's compiler catches hallucination errors that AI tools may introduce — making AI-assisted Rust development more reliable than AI-assisted Python development
- **Common use cases:** performance-critical CLI tools, inference/web servers, linters, data processing pipelines

---

## 2. PyO3 Installation

PyO3 is the bridge that allows Rust and Python to call each other.

**Prerequisites:** Rust, Python 3, a Python virtual environment

```bash
# Create and activate a virtual environment
python3 -m venv ~/.venv
source ~/.venv/bin/activate

# Install maturin (build tool for PyO3 projects)
pip install maturin

# Create a new PyO3 project
maturin new pyo3-example
cd pyo3-example

# Initialise with PyO3 bindings
maturin init  # select pyo3

# Build and import in Python
maturin develop
python
>>> import pyo3_example
>>> pyo3_example.sum_as_string(1, 2)
```

- `Cargo.toml` is auto-configured with the `pyo3` dependency and `cdylib` library type
- The build produces a `.so` shared object file that Python can import

---

## 3. Basic Rust Library for Python

- Write computationally expensive logic in Rust, expose it as a Python-importable `.so` file
- Annotate Rust functions with `#[pyfunction]` and wrap them in a `#[pymodule]`:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn calculate_pi(iterations: u64) -> f64 {
    let mut pi = 0.0;
    // ... calculation
    pi
}

#[pymodule]
fn libdigits_pi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    Ok(())
}
```

- Build with `cargo build --release` and copy the `.so` to your working directory
- Import in Python just like any other module:

```python
import libdigits_pi
result = libdigits_pi.calculate_pi(1_000_000)
```

- Use a **Makefile** to automate the build, clean, and copy steps
- Add `.so` files to `.gitignore`

---

## 4. Converting Rust Types to Python

- PyO3 provides Python native types (`PyDict`, `PyList`, `PyString`, etc.) that can be used directly in Rust:

```rust
#[pyfunction]
fn data_types_example(py: Python) -> PyResult<PyObject> {
    let text: &str = "hello";
    let integer: i32 = 42;
    let floating: f64 = 3.14;
    let boolean: bool = true;

    let dict = PyDict::new(py);
    dict.set_item("text", text)?;
    dict.set_item("integer", integer)?;
    dict.set_item("floating", floating)?;
    dict.set_item("boolean", boolean)?;
    Ok(dict.into())
}
```

- The resulting Python dictionary can be printed, iterated, and used exactly like a regular Python dict

---

## 5. Rust Ownership Model in Python

- Rust's explicit mutability can be exposed to Python via `#[pyclass]` and `#[pymethods]`:

```rust
#[pyclass]
struct NumberList {
    numbers: Vec<i32>,
}

#[pymethods]
impl NumberList {
    #[new]
    fn new() -> Self { NumberList { numbers: vec![] } }
    fn add(&mut self, n: i32) { self.numbers.push(n); }
    fn len(&self) -> usize { self.numbers.len() }
    fn clear(&mut self) { self.numbers.clear(); }
}
```

- Python code gets controlled access to Rust's strict mutability rules — useful for security-sensitive libraries

---

## 6. PyO3 Project Diagram (Recommended Workflow)

```
Rust (src/lib.rs)
  └── Heavy computation logic
  └── Exposed via #[pymodule] → builds libpycalc_cli.so

Makefile
  └── cargo build --release
  └── cp .so to working directory

Python (calc.py)
  └── import libpycalc_cli
  └── Wrap functions in a class
  └── Use Python Fire for instant CLI
```

- **Python Fire** (Google library) turns any Python class into a CLI with zero boilerplate:

```python
import fire
import libpycalc_cli

class Calculator:
    def add(self, a, b): return libpycalc_cli.add_as_string(a, b)
    def subtract(self, a, b): return libpycalc_cli.subtract_as_string(a, b)

if __name__ == "__main__":
    fire.Fire(Calculator)
```

```bash
./calc.py add 2 2
./calc.py subtract 5 2
```

---

## 7. PyO3 Features

| Feature                  | Description                                                            |
|--------------------------|------------------------------------------------------------------------|
| Rust extensions in Python | Write Python extensions in Rust — high performance, seamless import  |
| Rust calling Python       | Call Python scripts, functions, and manipulate Python objects from Rust|
| Native types              | Use `PyDict`, `PyList`, `PyString` as native Rust types               |
| Exception handling        | Raise and catch Python exceptions (`PyErr`) from Rust code            |

---

## 8. PyO3 Exception Handling

- Raise a Python exception from Rust:

```rust
use pyo3::exceptions::PyZeroDivisionError;

#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyZeroDivisionError::new_err("Cannot divide by zero"));
    }
    Ok(a / b)
}
```

- Catch in Python as a normal exception:

```python
try:
    result = librust_exceptions.divide(10, 0)
except ZeroDivisionError as e:
    print(f"Caught: {e}")
```

---

## 9. Calling Python from Rust

- PyO3 allows Rust to directly call Python code, including the standard library:

```rust
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let builtins = py.import("builtins")?;
        let sum: i32 = builtins.getattr("sum")?.call1(([1, 2, 3],))?.extract()?;
        println!("Sum: {}", sum);

        let os = py.import("os")?;
        let user: String = os.getattr("getenv")?.call1(("USER",))?.extract()?;
        println!("User: {}", user);
        Ok(())
    })
}
```

- Use `prepare_freethreaded_python()` to release the GIL for multi-threaded use
- Access any Python standard library module with `py.import("module_name")`

---

## 10. Embedded Python in Rust

- Embed a Python script directly inside Rust code:

```rust
Python::with_gil(|py| {
    let module = PyModule::from_code(py, "
def marco(input):
    if input == 'marco':
        return 'python'
    return 'no python'
", "marco.py", "marco")?;

    let result: String = module
        .getattr("marco")?
        .call1(("marco",))?
        .extract()?;
    println!("{}", result);
    Ok(())
})
```

- Great for reusing tested Python business logic within a Rust binary
- Combine with a **Makefile** workflow: `make format` → `make lint` → `make run`

---

## 11. Embedded Rust CLI with CLAP

- Wrap embedded Python logic in a CLI using the `clap` crate:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
pyo3 = { version = "0.x", features = ["auto-initialize"] }
```

- Benefits of a Rust CLI over Python CLI:
  - **Binary deployment** — no interpreter needed; ship a single executable
  - Cross-platform, fast startup, robust packaging via Cargo
  - Test with `cargo run -- --help` and `cargo run -- --input marco`

---

## 12. Testing Rust + Python Projects

**Unit tests** — validate business logic directly in `lib.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marco() {
        let result = call_embedded_python("marco").unwrap();
        assert_eq!(result, "python");
    }
}
```

**Integration tests** — test the CLI tool end-to-end from `tests/`:
```rust
use std::process::Command;

#[test]
fn test_cli_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--input", "marco"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("python"));
}
```

- Run all tests: `cargo test`
- A failing test reports exactly what was expected vs. what was returned

---

## 13. Rust-Built Python Tools

### Polars
- A **DataFrame library** written in Rust — use it from Python for massive performance gains over Pandas
- Supports parallel processing, multi-threading, and handles datasets Pandas cannot (e.g. 50GB+)
- Python API mirrors Pandas but with significantly faster execution

### Ruff
- A **Python linter** written in Rust — 10–100× faster than Pylint/Flake8
- Sub-second linting vs. 60+ seconds for pure Python linters on large codebases
- Use in CI/CD pipelines for near-instant feedback
- Install: `pip install ruff`; integrate via GitHub Actions for continuous linting

### Energy Efficiency
- Python uses ~75× more energy than Rust for equivalent tasks
- Rust-based tools (Polars, Ruff, etc.) reduce both runtime and energy costs at scale

---

## 14. Polars CLI with Benchmarking

**Project structure:**
```
polars-cli/
  ├── Cargo.toml        # clap + polars + criterion dependencies
  ├── Makefile          # build, run, test, bench commands
  ├── src/
  │   ├── lib.rs        # calculate() function with groupby/aggregation
  │   └── main.rs       # clap CLI wrapper
  ├── benches/
  │   └── my_benchmark.rs  # Criterion benchmarks
  └── tests/
      └── integration_test.rs
```

**Criterion benchmarking:**
```toml
[[bench]]
name = "my_benchmark"
harness = false

[dev-dependencies]
criterion = "0.5"
```

```bash
make bench  # runs Criterion, outputs HTML report in target/criterion/
```

**Benchmark comparison (iris dataset groupby):**
| Tool          | Time      |
|---------------|-----------|
| Rust + Polars | ~0.55s    |
| Python Polars | ~1.6s     |
| Python Pandas | ~4.8s     |

**Recommended Makefile commands:**
```makefile
build:    cargo build --release
run:      cargo run
lint:     cargo clippy
format:   cargo fmt
test:     cargo test
bench:    cargo bench
```