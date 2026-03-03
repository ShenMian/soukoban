use std::str::FromStr;

use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::Level;

use super::utils::*;

fn canonicalize(c: &mut Criterion) {
    c.bench_function("Map::canonicalize", |b| {
        let level = Level::from_str(WORLDCUP2014).unwrap();
        b.iter_batched_ref(
            || level.map().clone(),
            |map| map.canonicalize(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, canonicalize);
