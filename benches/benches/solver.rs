use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::{prelude::*, solver::*};

use super::utils::*;

fn a_star_search(c: &mut Criterion) {
    let mut bench_search = |level: Level, strategy: Strategy| {
        c.bench_function(
            &format!(
                "Solver::search '{}' ({:?}, {:?})",
                level.metadata()["title"],
                Algorithm::AStar,
                strategy
            ),
            |b| {
                b.iter_batched_ref(
                    || Solver::new(level.map().clone(), strategy),
                    |solver| solver.search(Algorithm::AStar).unwrap(),
                    BatchSize::SmallInput,
                );
            },
        );
    };

    let level = load_level_from_file("assets/Benchmark_3.xsb", 1);
    bench_search(level, Strategy::Quick);

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 3);
    bench_search(level, Strategy::Quick);
}

fn ida_star_search(c: &mut Criterion) {
    let mut bench_search = |level: Level, strategy: Strategy| {
        c.bench_function(
            &format!(
                "Solver::search '{}' ({:?}, {:?})",
                level.metadata()["title"],
                Algorithm::IDAStar,
                strategy
            ),
            |b| {
                b.iter_batched_ref(
                    || Solver::new(level.map().clone(), strategy),
                    |solver| solver.search(Algorithm::IDAStar).unwrap(),
                    BatchSize::SmallInput,
                );
            },
        );
    };

    let level = load_level_from_file("assets/Benchmark_3.xsb", 1);
    bench_search(level, Strategy::Quick);

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 3);
    bench_search(level, Strategy::Quick);
}

criterion_group!(benches, a_star_search, ida_star_search);
