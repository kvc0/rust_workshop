use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::single_thread,
}
