#[macro_use]
extern crate criterion;
extern crate segmented_vec;

use criterion::Criterion;
use segmented_vec::*;

const N: usize = 10_000;

fn std_vec_push(c: &mut Criterion) {
    c.bench_function("std::Vec::push", |b| b.iter(|| {
        let mut vec = Vec::new();
        for i in 0..N {
            vec.push([i as u64, 1, 2, 3, 4, 5, 6, 7, 8]);
        }
    }));
}

fn std_vec_push_preallocated(c: &mut Criterion) {
    c.bench_function("std::Vec::push preallocated", |b| b.iter(|| {
        let mut vec = Vec::with_capacity(N);
        for i in 0..N {
            vec.push([i as u64, 1, 2, 3, 4, 5, 6, 7, 8]);
        }
    }));
}

fn segmented_64_vec_push(c: &mut Criterion) {
    c.bench_function("SegmentedVec(64)::push", |b| b.iter(|| {
        let mut vec = SegmentedVec::with_chunk_size(64);
        for i in 0..N {
            vec.push([i as u64, 1, 2, 3, 4, 5, 6, 7, 8]);
        }
    }));
}

fn segmented_128_vec_push(c: &mut Criterion) {
    c.bench_function("SegmentedVec(128)::push", |b| b.iter(|| {
        let mut vec = SegmentedVec::with_chunk_size(128);
        for i in 0..N {
            vec.push([i as u64, 1, 2, 3, 4, 5, 6, 7, 8]);
        }
    }));
}

fn segmented_256_vec_push(c: &mut Criterion) {
    c.bench_function("SegmentedVec(256)::push", |b| b.iter(|| {
        let mut vec = SegmentedVec::with_chunk_size(256);
        for i in 0..N {
            vec.push([i as u64, 1, 2, 3, 4, 5, 6, 7, 8]);
        }
    }));
}

fn segmented_512_vec_push(c: &mut Criterion) {
    c.bench_function("SegmentedVec(512)::push", |b| b.iter(|| {
        let mut vec = SegmentedVec::with_chunk_size(512);
        for i in 0..N {
            vec.push([i as u64, 1, 2, 3, 4, 5, 6, 7, 8]);
        }
    }));
}

fn segmented_2048_vec_push(c: &mut Criterion) {
    c.bench_function("SegmentedVec(2048)::push", |b| b.iter(|| {
        let mut vec = SegmentedVec::with_chunk_size(2048);
        for i in 0..N {
            vec.push([i as u64, 1, 2, 3, 4, 5, 6, 7, 8]);
        }
    }));
}

criterion_group!(benches,
    std_vec_push,
    std_vec_push_preallocated,
    segmented_64_vec_push,
    segmented_128_vec_push,
    segmented_256_vec_push,
    segmented_512_vec_push,
    segmented_2048_vec_push
);
criterion_main!(benches);