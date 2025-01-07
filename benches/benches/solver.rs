use std::str::FromStr;

use criterion::{black_box, criterion_group, Criterion};
use soukoban::{
    solver::{Solver, Strategy},
    Level,
};

use super::utils::*;

fn a_star_search(c: &mut Criterion) {
    let mut bench_solve = |level: Level| {
        c.bench_function(
            &format!("Solver::a_star_search '{}'", level.metadata()["title"]),
            |b| {
                let solver = black_box(Solver::new(level.map().clone(), Strategy::Fast));
                b.iter(|| {
                    solver.a_star_search().unwrap();
                })
            },
        );
    };

    let level = Level::from_str(PATH).unwrap();
    bench_solve(level);

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 3);
    bench_solve(level);
}

fn ida_star_search(c: &mut Criterion) {
    let mut bench_solve = |level: Level| {
        c.bench_function(
            &format!("Solver::ida_star_search '{}'", level.metadata()["title"]),
            |b| {
                let solver = black_box(Solver::new(level.map().clone(), Strategy::Fast));
                b.iter(|| {
                    solver.ida_star_search().unwrap();
                })
            },
        );
    };

    let level = Level::from_str(PATH).unwrap();
    bench_solve(level);

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 3);
    bench_solve(level);
}

fn tunnels(c: &mut Criterion) {
    let level = Level::from_str(PATH).unwrap();
    let solver = Solver::new(level.map().clone(), Strategy::Fast);
    solver.lower_bounds();
    c.bench_function("Solver::tunnels", |b| {
        let solver = solver.clone();
        b.iter(|| {
            black_box(solver.tunnels());
        })
    });
}

criterion_group!(benches, a_star_search, ida_star_search, tunnels);
