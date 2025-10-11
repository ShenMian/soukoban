use std::hint::black_box;
use std::str::FromStr;

use criterion::{Criterion, criterion_group};
use soukoban::Map;

use super::utils::*;

fn normalize(c: &mut Criterion) {
    let map = Map::from_str(WORLDCUP2014).unwrap();
    c.bench_function("Map::normalize", |b| {
        b.iter(|| black_box(map.clone()).normalize())
    });
}

criterion_group!(benches, normalize);
