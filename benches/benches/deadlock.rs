use std::hint::black_box;
use std::str::FromStr;

use criterion::{Criterion, criterion_group};
use soukoban::{Map, deadlock};

use super::utils::*;

fn compute_unused_floors(c: &mut Criterion) {
    let map = Map::from_str(WORLDCUP2014).unwrap();
    c.bench_function("deadlock::compute_useless_floors", |b| {
        b.iter(|| black_box(deadlock::compute_useless_floors(black_box(map.clone()))))
    });
}

fn compute_static_deadlocks(c: &mut Criterion) {
    let map = Map::from_str(WORLDCUP2014).unwrap();
    c.bench_function("deadlock::compute_static_deadlocks", |b| {
        b.iter(|| black_box(deadlock::compute_static_deadlocks(black_box(&map))))
    });
}

criterion_group!(benches, compute_unused_floors, compute_static_deadlocks);
