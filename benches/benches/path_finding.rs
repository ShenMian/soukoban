use criterion::{Criterion, criterion_group};
use soukoban::{path_finding, prelude::*, solver::Strategy};

use super::utils::*;

fn compute_box_waypoints(c: &mut Criterion) {
    c.bench_function("path_finding::compute_box_waypoints '一箭十万'", |b| {
        b.iter_batched_ref(
            || load_level_from_file("assets/Benchmark_3.xsb", 1),
            |level| {
                path_finding::compute_box_waypoints(level.map(), Point::new(6, 4), Strategy::Quick)
            },
            criterion::BatchSize::SmallInput,
        );
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
                path_finding::compute_box_waypoints(level.map(), Point::new(4, 44), Strategy::Quick)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, compute_box_waypoints);
