use std::hash::Hash;
use webview_bundle::Bundle;

pub trait Cache<K: Hash + Eq, V> {
  fn has(&self, key: &K) -> bool;
  fn get(&mut self, key: &K) -> Option<&V>;
  fn set(&mut self, key: K, value: V);
}

#[derive(Clone, Default)]
pub struct NoopCache;

impl Cache<String, Bundle> for NoopCache {
  fn has(&self, _key: &String) -> bool {
    false
  }

  fn get(&mut self, _key: &String) -> Option<&Bundle> {
    None
  }

  fn set(&mut self, _key: String, _value: Bundle) {}
}

#[cfg(feature = "cache-lru")]
#[derive(Clone)]
pub struct LruCache<K: Hash + Eq, V> {
  cache: lru::LruCache<K, V>,
}

#[cfg(feature = "cache-lru")]
impl<K: Hash + Eq, V> LruCache<K, V> {
  pub fn new(size: usize) -> Self {
    Self {
      cache: lru::LruCache::<K, V>::new(
        std::num::NonZeroUsize::new(size).expect("size is not non zero"),
      ),
    }
  }

  pub fn unbounded() -> Self {
    Self {
      cache: lru::LruCache::<K, V>::unbounded(),
    }
  }
}

#[cfg(feature = "cache-lru")]
impl Cache<String, Bundle> for LruCache<String, Bundle> {
  fn has(&self, key: &String) -> bool {
    self.cache.contains(key)
  }

  fn get(&mut self, key: &String) -> Option<&Bundle> {
    self.cache.get(key)
  }

  fn set(&mut self, key: String, value: Bundle) {
    self.cache.put(key, value);
  }
}
