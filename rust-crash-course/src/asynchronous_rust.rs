#![deny(clippy::all)]

// Asynchronous programming in rust
// Supports async/await
// It requires runtime from thrid-party libraries
// Core functionality is included but crates are needed for extras

// Go to crates.io and search "futures" and "tokio" crates.
// Copy under the [dependencies] inside the cargo.toml:
// futures = "0.3.25"
// tokio = {version = "1.21.2", features = ["full"]}
// The version might change

use futures::executor::block_on;
use futures::Future;
use tokio::time::{sleep, Duration};

// Let's write an async function
async fn get_name() -> String {
    "John".to_string()
}

async fn call_api_one() -> String {
    sleep(Duration::from_secs(1)).await;
    "One".to_string()
}

async fn call_api_two() -> String {
    sleep(Duration::from_secs(1)).await;
    "Two".to_string()
}

// An async fn could just used the Future trait like this:
// Notice that the rust-analyzer will suggest the previous method though
fn call_api_one_future() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "One".to_string()
    }
}

fn call_api_two_future() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "Two".to_string()
    }
}

// The lifetime of variables returned by Future are tricky:
// They need to live as long as the Future itself
// When you use a variable outside the async block need to be carefull
// Usually you bundle variables inside the async block

// Async funcs can move variables when declared outside the async block
fn get_async_name() -> impl Future<Output = String> {
    let name = "John".to_string();
    async move { format!("Hello, {} Doe", name) }
}

// For the tokio functions need to turn the main into async
#[tokio::main]
async fn main() {
    // You can just call an async function without a wait call on it
    //let name = block_on(get_name());
    //println!("Hello, {}", name);

    // Using tokio's functions
    // This will call api one, wait 1 second and blocks the api 2
    let one = call_api_one().await;
    println!("{}", one);

    // Once the await of api 1 finishes, api is executed
    let two = call_api_two().await;
    println!("{}", two);

    // Using Future, the call is similar to the conventional funcs
    let one_future = call_api_one_future().await;
    println!("{}", one_future);
    let two_future = call_api_two_future().await;
    println!("{}", two_future);

    // Use the get_async
    let name_async = get_async_name().await;
    println!("{}", name_async);
}
