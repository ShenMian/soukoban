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

fn construct_player_path(c: &mut Criterion) {
    let map = load_level_from_file("assets/Benchmark_3.xsb", 1).into();
    let (waypoints, costs) =
        path_finding::compute_box_waypoints(&map, Point::new(6, 4), Strategy::Quick);
    let goal_position = *map.goal_positions().iter().next().unwrap();
    let to = find_best_waypoint(&costs, goal_position).unwrap();
    let box_path = path_finding::construct_box_path(to, &waypoints);

    c.bench_function("path_finding::construct_player_path '一箭十万'", |b| {
        b.iter(|| path_finding::construct_player_path(&map, map.player_position(), &box_path));
    });
}

/// Finds the cheapest reachable box state at `target`.
fn find_best_waypoint(
    costs: &FxHashMap<DirectedPosition, i32>,
    target: Point,
) -> Option<DirectedPosition> {
    let mut best = None;
    let mut min_cost = i32::MAX;
    for (&state, &cost) in costs {
        if state.position == target && cost < min_cost {
            min_cost = cost;
            best = Some(state);
        }
    }
    best
}

criterion_group!(benches, compute_box_waypoints, construct_player_path);
