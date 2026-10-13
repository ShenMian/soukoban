use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::{path_finding, prelude::*, solver::Strategy};

use super::utils::*;

fn compute_box_waypoints(c: &mut Criterion) {
    for strategy in [Strategy::PushOptimal, Strategy::MoveOptimal] {
        c.bench_function(
            &format!("path_finding::compute_box_waypoints '一箭十万' ({strategy:?})"),
            |b| {
                b.iter_batched_ref(
                    || load_level_from_file("assets/Benchmark_3.xsb", 1),
                    |level| {
                        path_finding::compute_box_waypoints(level.map(), Point::new(6, 4), strategy)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    for strategy in [Strategy::PushOptimal, Strategy::MoveOptimal] {
        c.bench_function(
            &format!("path_finding::compute_box_waypoints 'beemaze' ({strategy:?})"),
            |b| {
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
                            Point::new(4, 44),
                            strategy,
                        )
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
}

fn construct_player_path(c: &mut Criterion) {
    let map = load_level_from_file("assets/Benchmark_3.xsb", 1).into();
    let (waypoints, costs) =
        path_finding::compute_box_waypoints(&map, Point::new(6, 4), Strategy::PushOptimal);
    let goal_position = *map.goal_positions().iter().next().unwrap();
    let to = find_best_waypoint(&costs, goal_position).unwrap();
    let box_path = path_finding::construct_box_path(to, &waypoints);

    c.bench_function("path_finding::construct_player_path '一箭十万'", |b| {
        b.iter(|| path_finding::construct_player_path(&map, map.player_position(), &box_path));
    });
}

fn find_path(c: &mut Criterion) {
    let map: Map = load_level_from_file("assets/Microban_II_135.xsb", 132).into();
    let from = map.player_position();
    let to = Point::new(25, 21);

    c.bench_function("path_finding::find_path", |b| {
        b.iter(|| path_finding::find_path(from, to, |position| map.is_walkable(position)).unwrap());
    });
}

fn find_path_cost(c: &mut Criterion) {
    let map: Map = load_level_from_file("assets/Microban_II_135.xsb", 132).into();
    let from = map.player_position();
    let to = Point::new(25, 21);

    c.bench_function("path_finding::find_path_cost", |b| {
        b.iter(|| {
            path_finding::find_path_cost(from, to, |position| map.is_walkable(position)).unwrap()
        });
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

criterion_group!(
    benches,
    compute_box_waypoints,
    construct_player_path,
    find_path,
    find_path_cost
);
