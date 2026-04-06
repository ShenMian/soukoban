use std::collections::HashSet;

use criterion::{BatchSize, Criterion, criterion_group};
use nalgebra::Vector2;
use soukoban::deadlock;

use super::utils::*;

fn is_freeze_deadlock(c: &mut Criterion) {
    c.bench_function("deadlock::is_freeze_deadlock", |b| {
        let level = load_level_from_file("assets/Deadlock_3.xsb", 2);
        b.iter_batched(
            || level.map().clone(),
            |map| {
                deadlock::is_freeze_deadlock(
                    &map,
                    Vector2::new(3, 1),
                    map.box_positions(),
                    &mut HashSet::new(),
                )
            },
            BatchSize::SmallInput,
        )
    });
}

fn compute_useless_floors(c: &mut Criterion) {
    c.bench_function("deadlock::compute_useless_floors", |b| {
        let level = load_level_from_file("assets/Benchmark_3.xsb", 3);
        b.iter_batched(
            || level.map().clone(),
            deadlock::compute_useless_floors,
            BatchSize::SmallInput,
        )
    });
}

fn compute_static_deadlocks(c: &mut Criterion) {
    c.bench_function("deadlock::compute_static_deadlocks", |b| {
        let level = load_level_from_file("assets/Benchmark_3.xsb", 3);
        b.iter(|| deadlock::compute_static_deadlocks(level.map()))
    });
}

criterion_group!(
    benches,
    is_freeze_deadlock,
    compute_useless_floors,
    compute_static_deadlocks
);
