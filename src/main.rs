mod hashing;
mod redis;
mod server;

use std::sync::Arc;

use tokio::sync::Mutex;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    pretty_env_logger::init();

    let mut ring = hashing::Ring::new(5);

    ring.add_node(String::from("redis://localhost:6379"));
    ring.add_node(String::from("redis://localhost:6380"));
    ring.add_node(String::from("redis://localhost:6381"));

    let redis_store = redis::Store { ring: ring };

    warp::serve(server::handler(Arc::new(Mutex::new(redis_store))))
        .run(([127, 0, 0, 1], 8080))
        .await;

    Ok(())
}
