# Rust: Asynchronous Programming with Tokio

---

## 1. What is Asynchronous Programming?

- Async programming is about **interleaving tasks** and using the downtime from one task to make progress on another
- Async functions have two states: **pending** (still waiting) and **done** (completed with a return value)
- The **async runtime** periodically checks all running functions — if pending, it comes back later; if done, it collects the return value and moves on
- Real-life analogy: starting laundry (pending), then ordering takeout (pending) while you wait — you are the runtime, each task is an async function

---

## 2. When to Use Asynchronous Programming?

- **CPU-bound tasks** — the CPU must constantly work with no built-in downtime (e.g. heavy computation); async adds context-switching overhead and can make things **slower**
- **I/O-bound tasks** — the CPU spends time **waiting** on input/output (e.g. network requests, file reads); non-async programs waste this time; async uses it to make progress on other tasks
- Use async when your program has many **I/O-bound tasks**
- Avoid async when dominated by **CPU-bound tasks**

---

## 3. How async and await Work in Rust

- `async fn` marks a function as asynchronous:
  ```rust
  async fn hello_world() -> String {
      String::from("Hello, world!")
  }
  ```
- `.await` waits for an async function to finish and returns its value:
  ```rust
  let result = hello_world().await;
  ```
- Rust does **not** include an async runtime by default — use the **Tokio** crate
- Start the runtime with `#[tokio::main]` above your async main function:
  ```rust
  #[tokio::main]
  async fn main() {
      let result = hello_world().await;
      println!("{}", result);
  }
  ```

---

## 4. Tokio's Async Runtime

- `#[tokio::main]` starts a **multi-threaded runtime**, copies the main function body, and passes it to the runtime's `.block_on()` method
- Manually building the equivalent runtime looks like:
  ```rust
  tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()
      .unwrap()
      .block_on(async { /* main body here */ });
  ```
- `.enable_all()` enables all Tokio features (currently IO and time)
- The runtime is responsible for calling the first async function — nothing external calls `main` directly

---

## 5. How to Spawn a Task

- A **task** is a lightweight, non-blocking unit of execution running in a separate thread
- Spawn a task with `tokio::spawn`:
  ```rust
  let handle = tokio::spawn(hello(String::from("Marcus")));
  let result = handle.await.unwrap();
  ```
- `tokio::spawn` returns a **join handle** — a bridge between the spawned task and the main thread
- Call `.await` on the handle to wait for the task to finish and retrieve its return value
- Join handles return `Result<T, JoinError>` — unwrap to get the value; `JoinError` occurs on cancellation or panic
- You can spawn as many tasks as needed — just `await` all their handles

---

## 6. How to Spawn a Synchronous (Blocking) Task

- Non-async code inside async functions **starves** the runtime because it has no `await` points for the runtime to jump between tasks
- Use `tokio::task::spawn_blocking` to run non-async code in a **dedicated blocking thread** that doesn't share resources with async threads:
  ```rust
  let handle = tokio::task::spawn_blocking(blocking_call);
  let result = handle.await.unwrap();
  ```
- Blocking threads prevent starvation of async functions — async tasks continue making progress while the blocking task runs

---

## 7. How to Test Asynchronous Code

- Async functions can only be called from within another async function — test functions must also be async
- Use the `#[tokio::test]` macro instead of `#[test]`:
  ```rust
  #[tokio::test]
  async fn test_add() {
      let result = add(1, 1).await;
      assert_eq!(result, 2);
  }
  ```
- By default, `#[tokio::test]` creates a **single-threaded** runtime
- For multi-threaded tests, configure the macro:
  ```rust
  #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
  ```

---

## 8. Async Primitives

Async primitives are mechanisms used to **synchronise the running of async threads**. They include:

| Primitive   | Purpose                                                             |
|-------------|---------------------------------------------------------------------|
| **Mutex**   | Exclusive access to a shared resource — only one task at a time    |
| **Semaphore** | Limit concurrent access to a resource to N tasks at a time       |
| **Notify**  | Signal one or all waiting tasks that an event has occurred          |
| **Barrier** | Hold tasks at a rendezvous point until a set number have arrived    |
| **RwLock**  | Allow concurrent reads but exclusive writes                         |

---

## 9. Mutex

- A **Mutex** enforces that only one task can access a resource at a time
- Declare with `tokio::sync::Mutex`, wrap in `Arc` for thread-safe sharing:
  ```rust
  use tokio::sync::Mutex;
  use std::sync::Arc;

  let mutex = Arc::new(Mutex::new(resource));
  ```
- Request access with `.lock().await` — other tasks block until the lock is released:
  ```rust
  let mut guard = mutex.lock().await;
  *guard = new_value;
  ```
- The lock is automatically released when the guard goes out of scope

---

## 10. Semaphore

- A **Semaphore** limits concurrent access to a resource to N tasks at a time (like N bank tellers serving a queue)
- Declare with `tokio::sync::Semaphore`, wrap in `Arc`:
  ```rust
  let semaphore = Arc::new(Semaphore::new(4)); // 4 permits
  ```
- Acquire a permit — blocks if all permits are in use:
  ```rust
  let permit = semaphore.acquire().await.unwrap();
  drop(permit); // release when done
  ```
- Tasks exceeding the permit count wait in a queue until a permit becomes available

---

## 11. Notify

- **Notify** signals one or all waiting tasks that an event has occurred
- Clone and share the `Arc<Notify>` between tasks:
  ```rust
  use tokio::sync::Notify;
  let notify = Arc::new(Notify::new());
  ```
- **Receiver** — waits for a notification:
  ```rust
  notify.notified().await;
  ```
- **Transmitter** — sends a notification:
  ```rust
  notify.notify_one();    // signals one waiting task
  notify.notify_waiters(); // signals all waiting tasks
  ```
- Use `notify_one` when only one specific task should respond; use `notify_waiters` to broadcast to all

---

## 12. Barrier

- A **Barrier** holds tasks at a rendezvous point until a set number have arrived, then releases all of them together
- Used with `Notify` to control batching:
  ```rust
  use tokio::sync::Barrier;
  let barrier = Arc::new(Barrier::new(12)); // waits for 12 tasks
  let result = barrier.wait().await;
  ```
- `barrier.wait()` returns a `BarrierWaitResult` — one task per batch is designated the **leader** (`result.is_leader()`)
- Use the leader to send a `Notify` signal to release the next batch
- Barriers are typically paired with `Notify` to precisely control the flow of items

---

## 13. RwLock

- **RwLock** (Read-Write Lock) allows **concurrent reads** but **exclusive writes**:
  - Multiple readers can access at the same time
  - Only one writer can access at a time — no readers allowed while writing
- Declare with `tokio::sync::RwLock`, wrap in `Arc`:
  ```rust
  let document = Arc::new(RwLock::new(String::new()));
  ```
- Request read access:
  ```rust
  let reader = document.read().await;
  ```
- Request write access:
  ```rust
  let mut writer = document.write().await;
  writer.push_str("new content");
  ```
- **Warning:** if a task holds both a read and write lock, it can cause a **deadlock** — give each reader and writer its own task

---

## 14. Channels

Channels pass data between threads. Tokio provides four types:

| Channel       | Producers | Consumers | Notes                                      |
|---------------|-----------|-----------|---------------------------------------------|
| **Oneshot**   | 1         | 1         | Sends one message; transmitter destroyed after |
| **Mpsc**      | Many      | 1         | Common for collecting work from many tasks  |
| **Watch**     | 1         | Many      | Only delivers the **most recent** message   |
| **Broadcast** | Many      | Many      | All consumers receive every message; uses a buffer |

### Oneshot Channel
- Single message, one sender, one receiver — transmitter is destroyed after sending:
  ```rust
  let (tx, rx) = tokio::sync::oneshot::channel();
  tx.send("secret message").unwrap();
  let msg = rx.await.unwrap();
  ```
- Use `tokio::select!` to wait on multiple oneshot receivers and act on the first that arrives

### Mpsc Channel (Multi-Producer, Single-Consumer)
- Multiple senders, one receiver — channel closes when all transmitters are dropped:
  ```rust
  let (tx, mut rx) = tokio::sync::mpsc::channel(32);
  // clone tx for each producer
  while let Some(msg) = rx.recv().await {
      // process message
  }
  ```

### Watch Channel (Single-Producer, Multi-Consumer)
- Consumers only receive the **most recent** value — older messages are dropped:
  ```rust
  let (tx, rx) = tokio::sync::watch::channel(initial_value);
  tx.send(updated_value).unwrap();
  while rx.changed().await.is_ok() {
      let value = rx.borrow();
  }
  ```

### Broadcast Channel (Multi-Producer, Multi-Consumer)
- Every consumer receives every message; new receivers are created via `.subscribe()`:
  ```rust
  let (tx, mut rx) = tokio::sync::broadcast::channel(16); // buffer of 16
  let mut rx2 = tx.subscribe();
  tx.send("message").unwrap();
  ```
- If the buffer is full, the **oldest message is dropped** and a `lagged` error is returned to the receiver — handle this in your error handling logic