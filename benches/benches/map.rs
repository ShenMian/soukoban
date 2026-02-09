use std::str::FromStr;

use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::Map;

use super::utils::*;

fn canonicalize(c: &mut Criterion) {
    c.bench_function("Map::canonicalize", |b| {
        let map = Map::from_str(WORLDCUP2014).unwrap();
        b.iter_batched_ref(
            || map.clone(),
            |map| map.canonicalize(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, canonicalize);
