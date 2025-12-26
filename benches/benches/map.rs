use std::str::FromStr;

use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::Map;

use super::utils::*;

fn normalize(c: &mut Criterion) {
    c.bench_function("Map::normalize", |b| {
        let map = Map::from_str(WORLDCUP2014).unwrap();
        b.iter_batched_ref(|| map.clone(), |map| map.normalize(), BatchSize::SmallInput)
    });
}

criterion_group!(benches, normalize);
