# Async Dropx

A practical, safe, and easy-to-use crate for "async destructors" in Rust.

## Why this crate?

Rust's `Drop` trait is synchronous. If you need to perform async cleanup (like closing a network connection, flushing a buffer, or sending a goodbye message) when an object goes out of scope, you're out of luck with standard Rust.

`async-dropx` solves this by providing a wrapper `AsyncDropx<T>` that detects when your object is dropped and automatically spawns a background task on your async runtime to handle the cleanup.

## Features

- 🚀 **Simple API**: Just implement `AsyncDrop` and wrap your type.
- ⚡ **Runtime Agnostic**: Works with `tokio` and `async-std` (via feature flags).
- 🛡️ **Panic Safe**: Cleanup runs even if your thread panics.
- 🔧 **Zero Overhead**: The wrapper is a transparent newtype around your object.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
async-dropx = { version = "0.1.0", features = ["tokio"] } # or "async-std"
```

Implement `AsyncDrop`:

```rust
use async_dropx::{AsyncDrop, AsyncDropx};
use std::pin::Pin;
use std::future::Future;

struct DatabaseConnection;

impl AsyncDrop for DatabaseConnection {
    type Dropper = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn async_drop(self) -> Self::Dropper {
        Box::pin(async move {
            println!("Closing connection asynchronously...");
            // await something here
        })
    }
}

#[tokio::main]
async fn main() {
    let conn = AsyncDropx::new(DatabaseConnection);
    
    // Use `conn` transparently (Deref coercion)
    
} // <--- `conn` is dropped here, and the cleanup task is spawned!
```

## How it works

1. You wrap your type `T` in `AsyncDropx<T>`.
2. `AsyncDropx` implements `Deref` so you can use it just like `T`.
3. When `AsyncDropx` goes out of scope, its synchronous `Drop` implementation runs.
4. It takes ownership of `T` and calls `async_drop(T)`.
5. It spawns the resulting Future on the currently active runtime.

## Supported Runtimes

- **Tokio**: Enable feature `tokio`.
- **Async-std**: Enable feature `async-std`.

If no runtime is detected or enabled, the drop will fail silently (with an error log) to avoid crashing your program, but the cleanup will not run.

## Author
- Ben Santora - truelinux@yahoo.com
