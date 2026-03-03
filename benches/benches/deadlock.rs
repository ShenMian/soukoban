use std::str::FromStr;

use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::{Level, deadlock};

use super::utils::*;

fn compute_unused_floors(c: &mut Criterion) {
    c.bench_function("deadlock::compute_useless_floors", |b| {
        let level = Level::from_str(WORLDCUP2014).unwrap();
        b.iter_batched(
            || level.map().clone(),
            deadlock::compute_useless_floors,
            BatchSize::SmallInput,
        )
    });
}

fn compute_static_deadlocks(c: &mut Criterion) {
    c.bench_function("deadlock::compute_static_deadlocks", |b| {
        let level = Level::from_str(WORLDCUP2014).unwrap();
        b.iter(|| deadlock::compute_static_deadlocks(level.map()))
    });
}

criterion_group!(benches, compute_unused_floors, compute_static_deadlocks);
