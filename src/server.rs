use super::redis;

use bytes::Bytes;
use std::str;

use std::sync::Arc;
use tokio::sync::Mutex;

use warp::Filter;

use std::convert::Infallible;
pub fn handler(
  store: Arc<Mutex<redis::Store>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  handle_get_key(store.clone()).or(handle_put_key(store))
}

pub fn handle_put_key(
  store: Arc<Mutex<redis::Store>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("keys" / String)
    .and(warp::put())
    .and(with_store(store))
    .and(warp::body::content_length_limit(1024 * 16))
    .and(warp::body::bytes())
    .and_then(put_key)
}

pub async fn put_key(
  _key: String,
  _store: Arc<Mutex<redis::Store>>,
  body: Bytes,
) -> Result<impl warp::Reply, Infallible> {
  let bytes = body.to_vec();
  let str_body = str::from_utf8(&bytes).unwrap();
  Ok(warp::reply::json(&String::from(str_body)))
}

pub fn handle_get_key(
  store: Arc<Mutex<redis::Store>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("keys" / String)
    .and(warp::get())
    .and(with_store(store))
    .and_then(get_key)
}

pub async fn get_key(
  key: String,
  store: Arc<Mutex<redis::Store>>,
) -> Result<impl warp::Reply, Infallible> {
  let store_locked = store.lock().await;
  let node = store_locked.get(key).await;
  Ok(warp::reply::json(&node.unwrap().value))
}

fn with_store(
  store: Arc<Mutex<redis::Store>>,
) -> impl Filter<Extract = (Arc<Mutex<redis::Store>>,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || store.clone())
}
