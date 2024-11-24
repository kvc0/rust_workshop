//! This file contains a variety of tests to help you verify that you have
//! implemented the sieve cache correctly.
//!
//! In Rust, you write a test by annotating a test function with the `#[test]`
//! attribute. Like this:
//! ```
//! #[test]
//! fn my_test() {
//!     // ...
//! }
//! ```
//!
//! The test will fail if the test function panics and pass otherwise. Rust
//! also provides a couple of different macros to make writing assertions
//! easier. You will see the tests below using the `assert!` and `assert_eq!`
//! macros.
//!
//! It is also possible to put the tests directly in the crate itself! I have
//! avoided that here to keep the tests out of the way but normally you would
//! want to keep them adjacent to the code they are testing.
//!
//! Want to learn more about how to write tests in rust? The Rust Book has
//! [a chapter][0] that goes over writing tests in much more detail.
//!
//! [0]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html

use cache::{SizeLimitedCache, MAX_SIZE};
use sieve_cache::SieveCache;

// These first few tests check the basics:
// - If you set a value can you then get it back out again?
// - Adding a few values (but less than cache::MAX_SIZE) doesn't lose the
//   earlier one.
// - If you overwrite a value then the new entry should be returned.

#[test]
fn set_and_get_same_value() {
    let mut cache = SieveCache::new();

    cache.set("test", 32);
    assert_eq!(cache.get(&"test"), Some(32));
}

#[test]
fn set_multiple_get_original() {
    let mut cache = SieveCache::new();

    cache.set("a", 0);
    cache.set("b", 1);
    cache.set("c", 2);

    assert_eq!(cache.get(&"a"), Some(0));
}

#[test]
fn set_overwrite() {
    let mut cache = SieveCache::new();

    cache.set("a", 0);
    cache.set("a", 1);

    assert_eq!(cache.get(&"a"), Some(1));
}

// These later tests check that eviction is working properly. They do make the
// assumption that entries are scanned in the order that they were inserted.
// This should be true for most implementations.

#[test]
fn evict_first() {
    let mut cache = SieveCache::new();

    // This should fill up the cache to the maximum size.
    for i in 0..MAX_SIZE {
        cache.set(i, i);
    }

    // And this should cause the first entry to be evicted and nothing else.
    cache.set(MAX_SIZE, MAX_SIZE);

    assert_eq!(cache.get(&0), None);
    assert_eq!(cache.get(&1), Some(1));
}

#[test]
fn evict_skips_read_values() {
    let mut cache = SieveCache::new();

    // This should fill up the cache to the maximum size.
    for i in 0..cache::MAX_SIZE {
        cache.set(i, i);
    }

    // Read some values so that they get marked as having been read.
    cache.get(&0);
    cache.get(&1);

    // Now insert a new one to cause an eviction to happen.
    cache.set(MAX_SIZE, MAX_SIZE);

    assert_eq!(cache.get(&0), Some(0));
    assert_eq!(cache.get(&1), Some(1));
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&MAX_SIZE), Some(MAX_SIZE));
}

#[test]
fn evict_only_evicts_necessary_entries() {
    let mut cache = SieveCache::new();

    // This should fill up the cache to the maximum size.
    for i in 0..cache::MAX_SIZE {
        cache.set(i, i);
    }

    // Read some values so that they get marked as having been read.
    cache.get(&0);
    cache.get(&1);

    // Now insert a new one to cause an eviction to happen.
    cache.set(MAX_SIZE, MAX_SIZE);

    // Entry 2 should be evicted, but not entry 3
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&3), Some(3));
}
