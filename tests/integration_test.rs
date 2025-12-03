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
            // Simulate some work
            tokio::time::sleep(Duration::from_millis(10)).await;
            flag.notify_one();
        })
    }
}

#[tokio::test]
async fn test_async_drop_runs() {
    let flag = Arc::new(Notify::new());
    
    {
        let resource = TestResource { dropped_flag: flag.clone() };
        let _wrapper = AsyncDropx::new(resource);
        // wrapper goes out of scope here
    }

    // Wait for the drop to happen
    // We use a timeout to ensure we don't hang forever if it fails
    let result = tokio::time::timeout(Duration::from_secs(1), flag.notified()).await;
    
    assert!(result.is_ok(), "Async drop did not complete in time");
}
