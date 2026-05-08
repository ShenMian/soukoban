use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::prelude::*;

use super::utils::*;

fn canonicalize(c: &mut Criterion) {
    c.bench_function("Map::canonicalize", |b| {
        let level = load_level_from_file("assets/Benchmark_3.xsb", 3);
        b.iter_batched_ref(
            || level.map().clone(),
            Map::canonicalize,
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, canonicalize);
