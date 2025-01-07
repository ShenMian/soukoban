use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};
use soukoban::{deadlock, Map};

use super::utils::*;

fn calculate_unused_floors(c: &mut Criterion) {
    let map = Map::from_str(WORLDCUP2014).unwrap();
    c.bench_function("deadlock::calculate_unused_floors", |b| {
        b.iter(|| black_box(deadlock::calculate_unused_floors(black_box(map.clone()))))
    });
}

fn calculate_static_deadlocks(c: &mut Criterion) {
    let map = Map::from_str(WORLDCUP2014).unwrap();
    c.bench_function("deadlock::calculate_static_deadlocks", |b| {
        b.iter(|| black_box(deadlock::calculate_static_deadlocks(black_box(&map))))
    });
}

criterion_group!(benches, calculate_unused_floors, calculate_static_deadlocks);
