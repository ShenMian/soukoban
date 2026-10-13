use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::{prelude::*, solver::*};

use super::utils::*;

fn a_star_search(c: &mut Criterion) {
    let levels = [
        load_level_from_file("assets/Benchmark_3.xsb", 1),
        load_level_from_file("assets/BoxWorld_100.xsb", 3),
    ];
    let strategies = [
        Strategy::Quick,
        Strategy::PushOptimal,
        Strategy::MoveOptimal,
    ];

    for level in &levels {
        for strategy in strategies {
            bench_search(c, Algorithm::AStar, level, strategy);
        }
    }
}

fn ida_star_search(c: &mut Criterion) {
    let level = load_level_from_file("assets/Benchmark_3.xsb", 1);
    for strategy in [Strategy::Quick, Strategy::PushOptimal] {
        bench_search(c, Algorithm::IDAStar, &level, strategy);
    }

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 3);
    bench_search(c, Algorithm::IDAStar, &level, Strategy::Quick);
}

fn bfs_search(c: &mut Criterion) {
    let levels = [
        load_level_from_file("assets/Benchmark_3.xsb", 1),
        load_level_from_file("assets/BoxWorld_100.xsb", 3),
    ];

    // `Quick` and `PushOptimal` are identical under BFS; `MoveOptimal` is
    // not supported.
    for level in &levels {
        bench_search(c, Algorithm::Bfs, level, Strategy::PushOptimal);
    }
}

fn bench_search(c: &mut Criterion, algorithm: Algorithm, level: &Level, strategy: Strategy) {
    c.bench_function(
        &format!(
            "Solver::search '{}' ({:?}, {:?})",
            level.metadata()["title"],
            algorithm,
            strategy
        ),
        |b| {
            b.iter_batched_ref(
                || Solver::new(level.map().clone(), strategy),
                |solver| solver.search(algorithm).unwrap(),
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(benches, a_star_search, ida_star_search, bfs_search);
