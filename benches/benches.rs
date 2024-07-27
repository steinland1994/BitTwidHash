use criterion::{BenchmarkGroup, Criterion};

const FILEPATH: &str = "test-data/million.txt";

fn criterion_benchmark(c: &mut Criterion) {
    let bh0 = bittwidhash::BitTwidHash::new_with_secret(
        std::hash::RandomState::new().build_hasher().finish(),
    );
    // let bh1 = std::hash::RandomState::new();
    // let bh2 = fnv::FnvBuildHasher::default();
    // let bh3 = ahash::RandomState::new();
    // let bh4 = fxhash::FxBuildHasher::default();
    // let bh5 = Why {};
    let bh8 = Zwo {};

    let mut group0 = c.benchmark_group("Hashing");
    bench_single(&mut group0, &bh0, "BitTwidHash");
    // bench_single(&mut group0, &bh1, "Default");
    // bench_single(&mut group0, &bh2, "FNV");
    // bench_single(&mut group0, &bh3, "aHash");
    // bench_single(&mut group0, &bh4, "FxHash");
    // bench_single(&mut group0, &bh5, "WyHash");
    bench_single(&mut group0, &bh8, "ZwoHash");
    group0.finish();

    let lines = get_lines(10000);
    let mut group1 = c.benchmark_group("Multi Hashing");
    bench_from_vec(&mut group1, &bh0, "BitTwidHash", &lines);
    // bench_from_vec(&mut group1, &bh1, "Default", &lines);
    // bench_from_vec(&mut group1, &bh2, "FNV", &lines);
    // bench_from_vec(&mut group1, &bh3, "aHash", &lines);
    // bench_from_vec(&mut group1, &bh4, "FxHash", &lines);
    // bench_from_vec(&mut group1, &bh5, "WyHash", &lines);
    bench_from_vec(&mut group1, &bh8, "ZwoHash", &lines);
    group1.finish();
}

//============================================================================

use criterion::{black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkId};
use std::fs::File;
use std::hash::{BuildHasher, Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Why();

impl std::hash::BuildHasher for Why {
    type Hasher = wyhash::WyHash;
    fn build_hasher(&self) -> wyhash::WyHash {
        wyhash::WyHash::with_seed(42)
    }
}

struct Zwo();

impl std::hash::BuildHasher for Zwo {
    type Hasher = zwohash::ZwoHasher;
    fn build_hasher(&self) -> zwohash::ZwoHasher {
        zwohash::ZwoHasher::default()
    }
}

fn get_lines(n: usize) -> Vec<String> {
    let f = File::open(Path::new(FILEPATH)).expect("Couldn't open file");
    BufReader::new(f)
        .lines()
        .take(n)
        .flatten()
        .collect::<Vec<_>>()
}

fn hash_key<S: BuildHasher, K: Hash>(bh: &S, th: K) -> u64 {
    let mut hasher = bh.build_hasher();
    th.hash(&mut hasher);
    hasher.finish()
}

fn hash_multiple<S: BuildHasher, K: Hash>(bh: &S, th: &Vec<K>) -> u64 {
    let mut x: u64 = 0;
    for (_, line) in (0..1000).zip(th) {
        x ^= hash_key(bh, line);
    }
    x
}

fn bench_single<S: BuildHasher>(
    group: &mut BenchmarkGroup<WallTime>,
    bh: &S,
    benchmark_name: &str,
) {
    group.bench_function(BenchmarkId::new(benchmark_name, ""), |b| {
        b.iter(|| hash_key(bh, black_box(b"andreas was here!!!\n")))
    });
}
fn bench_from_vec<S: BuildHasher, K: Hash>(
    group: &mut BenchmarkGroup<WallTime>,
    bh: &S,
    benchmark_name: &str,
    vec: &Vec<K>,
) {
    group.bench_function(BenchmarkId::new(benchmark_name, ""), |b| {
        b.iter(|| hash_multiple(bh, vec))
    });
}

criterion_group!(hash_benchmark, criterion_benchmark);
criterion_main!(hash_benchmark);
