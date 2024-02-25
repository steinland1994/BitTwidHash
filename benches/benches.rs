use std::hash::{BuildHasher, Hash, Hasher, RandomState};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fnv::FnvBuildHasher;
use whyhash::WhyHash;

fn hash_key<S: BuildHasher, K: Hash>(bh: &S, th: K) -> u64 {
    let mut hasher = bh.build_hasher();
    th.hash(&mut hasher);
    hasher.finish()
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let bh0 = WhyHash::new();
    let bh1 = RandomState::new();
    let bh2 = FnvBuildHasher::default();
    let mut group = c.benchmark_group("Hashing");
    group.bench_function(BenchmarkId::new("WhyHash", ""), |b| {
        b.iter(|| hash_key(&bh0, black_box(b"chongo was here!\n")))
    });
    group.bench_function(BenchmarkId::new("Default", ""), |b| {
        b.iter(|| hash_key(&bh1, black_box(b"chongo was here!\n")))
    });
    group.bench_function(BenchmarkId::new("FNV", ""), |b| {
        b.iter(|| hash_key(&bh2, black_box(b"chongo was here!\n")))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
