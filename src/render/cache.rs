use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

use ratatui::layout::Rect;

pub struct RenderCache<K: Hash + Eq, V> {
    cache: HashMap<K, (V, Instant)>,
    ttl: Duration,
}

impl<K: Hash + Eq, V: Clone> RenderCache<K, V> {
    pub fn new(ttl_ms: u64) -> Self {
        Self { cache: HashMap::new(), ttl: Duration::from_millis(ttl_ms) }
    }

    pub fn get_or_compute<F: FnOnce() -> V>(&mut self, key: K, compute: F) -> V {
        let now = Instant::now();
        if let Some((val, ts)) = self.cache.get(&key) {
            if now.duration_since(*ts) < self.ttl { return val.clone(); }
        }
        let val = compute();
        self.cache.insert(key, (val.clone(), now));
        val
    }

    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &K) { self.cache.remove(key); }
    #[allow(dead_code)]
    pub fn clear(&mut self) { self.cache.clear(); }
}

pub struct LazyValue<T> {
    value: Option<T>,
    dirty: bool,
}

impl<T> Default for LazyValue<T> {
    fn default() -> Self { Self { value: None, dirty: true } }
}

impl<T> LazyValue<T> {
    #[allow(dead_code)]
    pub fn get_or_compute<F: FnOnce() -> T>(&mut self, compute: F) -> &T {
        if self.dirty || self.value.is_none() {
            self.value = Some(compute());
            self.dirty = false;
        }
        self.value.as_ref().expect("value computed")
    }

    #[allow(dead_code)]
    pub fn mark_dirty(&mut self) { self.dirty = true; }
    #[allow(dead_code)]
    pub fn is_dirty(&self) -> bool { self.dirty }
}

pub struct LayoutCache {
    cache: HashMap<(u16, u16), Vec<Rect>>,
}

impl LayoutCache {
    #[allow(dead_code)]
    pub fn new() -> Self { Self { cache: HashMap::new() } }

    #[allow(dead_code)]
    pub fn get_or_compute(&mut self, area: Rect, compute: impl FnOnce(Rect) -> Vec<Rect>) -> Vec<Rect> {
        let key = (area.width, area.height);
        self.cache.entry(key).or_insert_with(|| compute(area)).clone()
    }

    #[allow(dead_code)]
    pub fn invalidate(&mut self) { self.cache.clear(); }
}
