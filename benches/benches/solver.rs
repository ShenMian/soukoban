use criterion::{BatchSize, Criterion, criterion_group};
use soukoban::{
    Level,
    solver::{Solver, Strategy},
};

use super::utils::*;

fn a_star_search(c: &mut Criterion) {
    let mut bench_search = |level: Level, strategy: Strategy| {
        c.bench_function(
            &format!(
                "Solver::a_star_search '{}' using '{:?}'",
                level.metadata()["title"],
                strategy
            ),
            |b| {
                b.iter_batched_ref(
                    || Solver::new(level.map().clone(), strategy),
                    |solver| solver.a_star_search().unwrap(),
                    BatchSize::SmallInput,
                )
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
                "Solver::ida_star_search '{}' using '{:?}'",
                level.metadata()["title"],
                strategy
            ),
            |b| {
                b.iter_batched_ref(
                    || Solver::new(level.map().clone(), strategy),
                    |solver| solver.ida_star_search().unwrap(),
                    BatchSize::SmallInput,
                )
            },
        );
    };

    let level = load_level_from_file("assets/Benchmark_3.xsb", 1);
    bench_search(level, Strategy::Quick);

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 3);
    bench_search(level, Strategy::Quick);
}

fn lower_bounds(c: &mut Criterion) {
    let level = load_level_from_file("assets/Aymeric_Du_Peloux_282.xsb", 67);
    let solver = Solver::new(level.map().clone(), Strategy::Quick);
    c.bench_function("Solver::lower_bounds", |b| {
        b.iter_batched_ref(
            || solver.clone(),
            |solver| {
                solver.lower_bounds();
            },
            BatchSize::SmallInput,
        )
    });
}

fn tunnels(c: &mut Criterion) {
    let level = load_level_from_file("assets/Benchmark_3.xsb", 1);
    let solver = Solver::new(level.map().clone(), Strategy::Quick);
    solver.lower_bounds();
    c.bench_function("Solver::tunnels", |b| {
        b.iter_batched_ref(
            || solver.clone(),
            |solver| {
                solver.tunnels();
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    a_star_search,
    ida_star_search,
    lower_bounds,
    tunnels
);
