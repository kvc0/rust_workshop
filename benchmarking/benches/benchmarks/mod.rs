use cache::synchronized_cache;
use criterion::{criterion_group, BenchmarkId, Criterion};
use kcache::{KCache, SharableKCache};
use lru_cache::LruCache;
use multi_thread_cache_test::benchmark_cache_multi_threaded;
use pprof::criterion::{Output, PProfProfiler};
use sieve_cache::ConcurrentSieveCache;
use single_thread_cache_test::benchmark_cache_single_threaded;

mod kcache;
mod lru_cache;
mod multi_thread_cache_test;
mod single_thread_cache_test;

fn single_threaded_comparison(c: &mut Criterion) {
    let mut single_thread_benchmark_group = c.benchmark_group("single_thread");

    benchmark_cache_single_threaded(
        BenchmarkId::from_parameter("workshop"),
        &mut single_thread_benchmark_group,
        sieve_cache::SieveCache::new(),
    );

    benchmark_cache_single_threaded(
        BenchmarkId::from_parameter("example"),
        &mut single_thread_benchmark_group,
        example_sieve_cache::SieveCache::new(),
    );

    benchmark_cache_single_threaded(
        BenchmarkId::from_parameter("k-cache"),
        &mut single_thread_benchmark_group,
        KCache::new(),
    );

    benchmark_cache_single_threaded(
        BenchmarkId::from_parameter("lru"),
        &mut single_thread_benchmark_group,
        LruCache::new(),
    );
}

fn multi_threaded_comparison(c: &mut Criterion) {
    let mut multi_thread_benchmark_group = c.benchmark_group("multi_thread");

    for thread_count in [1, 2, 4, 8, 12, 16] {
        benchmark_cache_multi_threaded(
            BenchmarkId::new("workshop", thread_count),
            &mut multi_thread_benchmark_group,
            thread_count,
            synchronized_cache(sieve_cache::SieveCache::new()),
        );

        benchmark_cache_multi_threaded(
            BenchmarkId::new("example", thread_count),
            &mut multi_thread_benchmark_group,
            thread_count,
            synchronized_cache(example_sieve_cache::SieveCache::new()),
        );

        benchmark_cache_multi_threaded(
            BenchmarkId::new("k-cache", thread_count),
            &mut multi_thread_benchmark_group,
            thread_count,
            SharableKCache::new(),
        );

        benchmark_cache_multi_threaded(
            BenchmarkId::new("lru", thread_count),
            &mut multi_thread_benchmark_group,
            thread_count,
            LruCache::new(),
        );

        benchmark_cache_multi_threaded(
            BenchmarkId::new("concurrent_sieve", thread_count),
            &mut multi_thread_benchmark_group,
            thread_count,
            ConcurrentSieveCache::new(),
        );
    }
}

criterion_group!(single_thread, single_threaded_comparison);
criterion_group! {
    name = multi_thread;
    config = Criterion::default().with_profiler(PProfProfiler::new(20000, Output::Flamegraph(None)));
    targets = multi_threaded_comparison
}
