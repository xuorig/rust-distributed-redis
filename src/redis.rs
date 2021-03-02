use super::hashing::Ring;

use redis::AsyncCommands;

use log::info;

pub struct GetResult {
  pub node: String,
  pub key: String,
  pub value: String,
}

pub struct Store {
  pub ring: Ring,
}

impl Store {
  pub async fn get(&self, key: String) -> Option<GetResult> {
    let node = self.ring.get(key.clone()).unwrap();

    info!("Connection to node {}", node.clone());

    let client = redis::Client::open(node.clone()).unwrap();

    info!("Client: {:?}", client);

    let mut con = client
      .get_async_connection()
      .await
      .expect("Could not connect to redis");

    let value = con.get::<String, String>(key.clone()).await.unwrap();

    Some(GetResult {
      node: node.clone(),
      key: key.clone(),
      value: value,
    })
  }
}
