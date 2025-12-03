use async_dropx::{AsyncDrop, AsyncDropx};
use std::sync::Arc;
use std::time::Duration;
use std::pin::Pin;
use std::future::Future;
// We can use a channel or just a simple sleep check for async-std
// async-std doesn't have a Notify equivalent in std, but we can use a channel.
use async_std::channel;

struct TestResource {
    sender: channel::Sender<()>,
}

impl AsyncDrop for TestResource {
    type Dropper = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn async_drop(self) -> Self::Dropper {
        let sender = self.sender.clone();
        Box::pin(async move {
            async_std::task::sleep(Duration::from_millis(10)).await;
            let _ = sender.send(()).await;
        })
    }
}

#[async_std::test]
async fn test_async_drop_runs_async_std() {
    let (sender, receiver) = channel::bounded(1);
    
    {
        let resource = TestResource { sender };
        let _wrapper = AsyncDropx::new(resource);
        // wrapper goes out of scope here
    }

    // Wait for the drop to happen
    let result = async_std::future::timeout(Duration::from_secs(1), receiver.recv()).await;
    
    assert!(result.is_ok(), "Async drop did not complete in time");
}
