#[macro_use]
extern crate criterion;
extern crate segmented_vec;

use criterion::Criterion;

mod simple;
mod std_vec;

criterion_main!(std_vec::std_benches, simple::simple_benches);
