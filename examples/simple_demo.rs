use async_dropx::{AsyncDrop, AsyncDropx};
use std::pin::Pin;
use std::future::Future;
use std::time::Duration;

// 1. Define your resource
struct MyResource {
    name: String,
}

// 2. Implement AsyncDrop for it
impl AsyncDrop for MyResource {
    // Boilerplate for now: this defines the future type
    type Dropper = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn async_drop(self) -> Self::Dropper {
        let name = self.name.clone();
        Box::pin(async move {
            println!("Cleaning up resource: {}", name);
            // Simulate async work
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Cleanup done for: {}", name);
        })
    }
}

#[tokio::main]
async fn main() {
    println!("Creating resource...");
    {
        let _res = AsyncDropx::new(MyResource { name: "Resource 1".to_string() });
        println!("Resource created. Exiting scope now...");
    }
    // The drop happens in the background.
    // In a real app, the runtime keeps running.
    // Here we sleep to let the background task finish before main exits.
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Main finished.");
}
