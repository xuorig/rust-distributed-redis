use crc::crc32;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ring {
  replicas: i32,
  ring: HashMap<u32, String>,
  keys: Vec<u32>,
}

impl Ring {
  /// Returns a HashRing given a set of nodes and a replication factor
  ///
  /// # Arguments
  ///
  /// * `nodes` - A Vec of Strings to distribute in the ring
  /// * `factor` - The number of virtual nodes in the ring for one node / aka `weight`
  ///
  pub fn new(replicas: i32) -> Ring {
    Ring {
      replicas: replicas,
      ring: HashMap::new(),
      keys: vec![],
    }
  }

  pub fn add_node(&mut self, node: String) {
    for i in 0..self.replicas {
      let node = node.clone();
      let key = format!("{}-{}", node, i.to_string());
      let hash = crc32::checksum_ieee(key.as_bytes());

      self.ring.insert(hash, node);
      self.keys.push(hash);
      self.keys.sort();
    }
  }

  /// Returns a Node identifier given a key
  ///
  /// * `key` the key for which we want the corresponding shard/node
  ///
  pub fn get(&self, key: String) -> Option<&String> {
    if self.ring.is_empty() {
      return None;
    }

    let hash = crc32::checksum_ieee(key.as_bytes());

    // Binary search for the closest node for this hash
    // Rust binary search returns an error with the index where we should
    // insert to keep the list sorted, pretty cool 👇
    let idx = match self.keys.binary_search(&hash) {
      Ok(i) => i,
      Err(i) => i,
    };

    // If we would need to insert at the end, pick the first node
    // instead.
    if idx == self.keys.len() {
      self.ring.get(&self.keys[0])
    } else {
      self.ring.get(&self.keys[idx])
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_node_replicates_with_factor() {
    let mut ring = Ring::new(5);
    ring.add_node(String::from("a"));
    ring.add_node(String::from("b"));
    assert_eq!(10, ring.ring.len());
  }

  #[test]
  fn get_empty_ring() {
    let ring = Ring::new(5);
    assert_eq!(None, ring.get(String::from("a")));
  }

  #[test]
  fn get_ring_one_node() {
    let mut ring = Ring::new(5);
    let node = String::from("a");
    ring.add_node(node.clone());
    let got_node = ring.get(String::from("my_key")).unwrap();
    assert_eq!(node, *got_node)
  }
}
