# async-dropx

**Async-aware cleanup for Rust — a practical experiment in async destructors.**

[![Crates.io](https://img.shields.io/crates/v/async-dropx.svg)](https://crates.io/crates/async-dropx)
[![Documentation](https://docs.rs/async-dropx/badge.svg)](https://docs.rs/async-dropx)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Rust does not currently support `async fn drop(&mut self)` at the language level.
`async-dropx` is an attempt to fill that gap with a **practical**, **runtime-safe**, and **ergonomic** pattern.

This crate provides:

* an `AsyncDrop` trait for async cleanup
* a wrapper type that ensures cleanup futures are executed
* executor integrations (Tokio first; others planned)
* predictable semantics for value teardown in async contexts

It is intentionally small, focused, and experimental.

---

## ✨ Example

```rust
use async_dropx::{AsyncDrop, AsyncOwned};

struct Conn {
    // ...
}

#[async_trait::async_trait]
impl AsyncDrop for Conn {
    async fn async_drop(&mut self) {
        // graceful async cleanup
        self.flush().await;
        self.shutdown().await;
    }
}

#[tokio::main]
async fn main() {
    let conn = AsyncOwned::new(Conn::new().await);

    // ... use conn ...

} // `async_drop()` will be awaited before destruction
```

---

## 🚧 Status: Early Preview

`async-dropx` is an experiment exploring async-aware destructors in stable Rust.
It passed internal tests, but **async cleanup is fundamentally tricky**, and there are edge cases we cannot fully eliminate.

Please expect:

* Differences across runtimes (Tokio vs async-std vs smol)
* Cleanup futures not running if the runtime is in shutdown
* Cleanup futures being cancelled if the surrounding task is cancelled
* Possible API refinements as the crate matures
* Certain patterns that “feel like they should work” but don’t (yet)

**This is new, there will be bugs, and your feedback matters.**

If you hit surprising behavior, hangs, or missing drops, **open an issue** with a snippet.
These real-world reports are incredibly valuable.

---

## 🧠 Why This Crate Exists

Rust has:

* `Drop` (sync, non-async)
* async runtimes with futures and cancellation
* network/file/task types that require async teardown

But **Rust has no native async destructors**, and likely won’t for a while.
This crate aims for an *80% practical* solution that bridges the gap safely.

---

## 🔍 How It Works (Short Version)

`AsyncOwned<T>` wraps a value implementing `AsyncDrop`, and on drop:

1. schedules the cleanup future on the active runtime
2. ensures the future is polled to completion (unless cancelled)
3. prevents double execution of cleanup
4. preserves `Drop` order for nested resources

The wrapper handles runtime interaction and ensures cleanup isn't silently forgotten.

---

## 📦 Feature Flags

* `tokio` — enable Tokio runtime integration (enabled by default)
* `async-std` — optional support
* `smol` — planned
* `derive` — optional proc-macro for `#[derive(AsyncDrop)]` (planned)

---

## 🧪 Testing Notes

Async teardown involves:

* runtime state
* shutdown logic
* cancellation
* panic unwinding

Tests cover the common expected cases, but unusual edge cases may still appear.
If you rely on strict ordering or guaranteed cleanup, evaluate carefully.

---

## 🗺 Roadmap

* [ ] Multi-runtime support
* [ ] Cancellation-safe drop modes
* [ ] Policies for “spawn vs block” cleanup
* [ ] Stronger guarantees around runtime shutdown
* [ ] Better diagnostics for missed drops
* [ ] Proc-macro derive for easier adoption

---

## 🤝 Contributing

Contributions are welcome — tests, examples, design discussions, issues, PRs, all of it.
If your workload reveals surprising behavior, please report it.

This crate is only useful if it works for the community using it.

---

## 📄 License

Dual-licensed under MIT or Apache 2.0.
Choose either license at your discretion.

---

Thanks for trying `async-dropx`.
The Rust community has wanted async destructors for a long time — this is one step toward them.
