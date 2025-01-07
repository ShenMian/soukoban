use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};
use nalgebra::Vector2;
use soukoban::{path_finding, Level};

use super::utils::*;

fn box_move_waypoints(c: &mut Criterion) {
    let level = Level::from_str(PATH).unwrap();
    c.bench_function("path_finding::box_move_waypoints", |b| {
        b.iter(|| path_finding::box_move_waypoints(black_box(level.map()), Vector2::new(6, 4)))
    });
}

criterion_group!(benches, box_move_waypoints);
