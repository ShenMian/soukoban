use std::str::FromStr;

use criterion::{Criterion, criterion_group};
use nalgebra::Vector2;
use soukoban::{Level, direction::Direction, path_finding};

use super::utils::*;

fn box_move_waypoints(c: &mut Criterion) {
    c.bench_function("path_finding::box_move_waypoints '一箭十万'", |b| {
        b.iter_batched_ref(
            || Level::from_str(PATH_1).unwrap(),
            |level| path_finding::box_move_waypoints(level.map(), Vector2::new(6, 4)),
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("path_finding::box_move_waypoints 'Microban 3, #101'", |b| {
        b.iter_batched_ref(
            || Level::from_str(PATH_2).unwrap(),
            |level| path_finding::box_move_waypoints(level.map(), Vector2::new(43, 6)),
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("path_finding::box_move_waypoints 'beemaze'", |b| {
        b.iter_batched_ref(
            || {
                let mut level = Level::from_str(PATH_3).unwrap();
                level
                    .execute_batch([Direction::Right, Direction::Right, Direction::Up])
                    .unwrap();
                level
            },
            |level| path_finding::box_move_waypoints(level.map(), Vector2::new(4, 44)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, box_move_waypoints);
