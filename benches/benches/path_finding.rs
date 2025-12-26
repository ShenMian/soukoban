use std::str::FromStr;

use criterion::{Criterion, criterion_group};
use nalgebra::Vector2;
use soukoban::{Level, path_finding};

use super::utils::*;

fn box_move_waypoints(c: &mut Criterion) {
    c.bench_function("path_finding::box_move_waypoints", |b| {
        let level = Level::from_str(PATH).unwrap();
        b.iter(|| path_finding::box_move_waypoints(level.map(), Vector2::new(6, 4)))
    });
}

criterion_group!(benches, box_move_waypoints);
