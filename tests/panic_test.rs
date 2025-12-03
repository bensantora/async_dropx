use async_dropx::{AsyncDrop, AsyncDropx};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;
use std::pin::Pin;
use std::future::Future;

struct TestResource {
    dropped_flag: Arc<Notify>,
}

impl AsyncDrop for TestResource {
    type Dropper = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn async_drop(self) -> Self::Dropper {
        let flag = self.dropped_flag.clone();
        Box::pin(async move {
            flag.notify_one();
        })
    }
}

#[tokio::test]
async fn test_async_drop_runs_on_panic() {
    let flag = Arc::new(Notify::new());
    let flag_clone = flag.clone();

    // Spawn a task that panics
    let handle = tokio::spawn(async move {
        let resource = TestResource { dropped_flag: flag_clone };
        let _wrapper = AsyncDropx::new(resource);
        panic!("Oops!");
    });

    // The task should panic
    let _ = handle.await;

    // But the drop should still run
    let result = tokio::time::timeout(Duration::from_secs(1), flag.notified()).await;
    assert!(result.is_ok(), "Async drop did not run after panic");
}
