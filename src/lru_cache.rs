extern crate lru;

use lru::LruCache;
use std::num::NonZeroUsize;
use crate::log_model::LogLevel;

pub fn sample_cache() {
    let mut cache = LruCache::new(NonZeroUsize::new(3).unwrap());
    // cache.put("apple", LogLevel::TRACE);
    cache.put("apple", 3);
    cache.put("banana", 2);
    cache.put("cucumber", 4);

    print!("------------------\n");
    if let Some(value) = cache.get(&"apple") {
        assert_eq!(*value, 3);
    } else {
        print!("apple not found\n");
    }

    print!("------------------\n");
    if let Some(value) = cache.get(&"banana") {
        assert_eq!(*value, 2);
    } else {
        print!("banana not found\n");
    }

    cache.iter().for_each(|(key, value)| {
        print!("{}: {}\n", key, value);
    });
}