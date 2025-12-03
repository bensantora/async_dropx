use std::ops::{Deref, DerefMut};
use std::future::Future;

/// Trait for types that require async cleanup.
///
/// Implement this trait for your type to define how it should be cleaned up asynchronously.
pub trait AsyncDrop {
    /// The future returned by `async_drop`.
    type Dropper: Future<Output = ()> + Send + 'static;

    /// Perform the async cleanup.
    /// This method consumes the object.
    fn async_drop(self) -> Self::Dropper;
}

/// Wrapper that ensures `async_drop` is called when the object goes out of scope.
///
/// This wrapper implements `Deref` and `DerefMut`, so you can use it just like the inner type.
/// When it goes out of scope, `Drop` is called, which takes the inner value and spawns
/// the future returned by `async_drop` on the active async runtime.
///
/// Supported runtimes:
/// - `tokio` (requires `tokio` feature)
/// - `async-std` (requires `async-std` feature)
pub struct AsyncDropx<T: AsyncDrop + Send + 'static> {
    inner: Option<T>,
}

impl<T: AsyncDrop + Send + 'static> AsyncDropx<T> {
    /// Create a new `AsyncDropx` wrapping the given value.
    pub fn new(inner: T) -> Self {
        Self { inner: Some(inner) }
    }
}

impl<T: AsyncDrop + Send + 'static> Deref for AsyncDropx<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().expect("Inner value is missing - this should never happen unless already dropped")
    }
}

impl<T: AsyncDrop + Send + 'static> DerefMut for AsyncDropx<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().expect("Inner value is missing - this should never happen unless already dropped")
    }
}

impl<T: AsyncDrop + Send + 'static> Drop for AsyncDropx<T> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            let _future = inner.async_drop();
            
            #[cfg(feature = "tokio")]
            {
                if let Ok(handle) = tokio::runtime::Handle::try_current() {
                    handle.spawn(_future);
                    return;
                }
            }
            
            #[cfg(feature = "async-std")]
            {
                // async-std doesn't strictly require a handle check, but we can just spawn it.
                // However, if we are not in a runtime, this might panic or fail?
                // async_std::task::spawn usually works if the runtime is initialized.
                async_std::task::spawn(_future);
                return;
            }

            // If we reach here, we couldn't spawn the task.
            // This might happen if:
            // 1. No feature flags are enabled.
            // 2. Tokio feature is enabled but we are not in a Tokio context.
            // 3. Async-std feature is enabled but something went wrong (though async-std is global).
            
            #[cfg(not(any(feature = "tokio", feature = "async-std")))]
            {
                eprintln!("AsyncDropx: No async runtime feature enabled (tokio/async-std). Cleanup leaked.");
            }
            
            #[cfg(any(feature = "tokio", feature = "async-std"))]
            {
                 eprintln!("AsyncDropx: Failed to spawn async cleanup task. Runtime might be missing or shut down.");
            }
        }
    }
}
