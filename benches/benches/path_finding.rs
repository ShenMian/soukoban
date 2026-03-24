use criterion::{Criterion, criterion_group};
use nalgebra::Vector2;
use soukoban::{direction::Direction, path_finding, solver::Strategy};

use super::utils::*;

fn compute_box_waypoints(c: &mut Criterion) {
    c.bench_function("path_finding::compute_box_waypoints '一箭十万'", |b| {
        b.iter_batched_ref(
            || load_level_from_file("assets/Benchmark_3.xsb", 1),
            |level| {
                path_finding::compute_box_waypoints(level.map(), Vector2::new(6, 4), Strategy::Fast)
            },
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("path_finding::compute_box_waypoints 'beemaze'", |b| {
        b.iter_batched_ref(
            || {
                let mut level = load_level_from_file("assets/Benchmark_3.xsb", 2);
                level
                    .execute_batch([Direction::Right, Direction::Right, Direction::Up])
                    .unwrap();
                level
            },
            |level| {
                path_finding::compute_box_waypoints(
                    level.map(),
                    Vector2::new(4, 44),
                    Strategy::Fast,
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, compute_box_waypoints);
