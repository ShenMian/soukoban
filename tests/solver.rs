use std::str::FromStr;

use nalgebra::Vector2;
use soukoban::{Actions, Level, solver::*};

mod utils;
use utils::*;

#[test]
fn a_star_search() {
    fn search(mut level: Level, strategy: Strategy) -> Actions {
        let solver = Solver::new(level.map().clone(), strategy);
        let solution = solver.a_star_search().unwrap();

        let directions = solution.iter().map(|action| action.direction());
        level.execute_batch(directions).unwrap();
        assert!(level.is_solved());

        solution
    }

    search(
        load_level_from_file("assets/BoxWorld_100.xsb", 1),
        Strategy::Fast,
    );
    search(
        load_level_from_file("assets/BoxWorld_100.xsb", 2),
        Strategy::Fast,
    );
    search(
        load_level_from_file("assets/BoxWorld_100.xsb", 3),
        Strategy::Fast,
    );

    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 1),
            Strategy::OptimalPush,
        )
        .pushes(),
        Actions::from_str("DuLLrUUdrR").unwrap().pushes()
    );
    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 1),
            Strategy::OptimalMove,
        )
        .moves(),
        Actions::from_str("DuLLrUUdrR").unwrap().moves()
    );

    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 2),
            Strategy::OptimalPush,
        )
        .pushes(),
        Actions::from_str(
            "rr4DrddlluRdrUl5ulldRur4D3RdrUUd3lddlluRdrUl4ulldRur3D3RdrU3lddlluRdrUlu3R"
        )
        .unwrap()
        .pushes()
    );
    // FIXME:
    // assert_eq!(
    //     search(
    //         load_level_from_file("assets/BoxWorld_100.xsb", 2),
    //         Strategy::OptimalMove,
    //     )
    //     .moves(),
    //     Actions::from_str(
    //         "rr4DrddlluRdrUl5ulldRur4D3RdrUUd3lddlluRdrUl4ulldRur3D3RdrU3lddlluRdrUlu3R"
    //     )
    //     .unwrap()
    //     .moves()
    // );

    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 3),
            Strategy::OptimalPush,
        )
        .pushes(),
        Actions::from_str(
            "rRRddrruULuu4l3D3u4rdd3L3ruu4ldDldRu6ruLd5luu4rDrd4LDu3ruu4ldDldRu3rddrUru4L3ruu4ldD"
        )
        .unwrap()
        .pushes()
    );
    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 3),
            Strategy::OptimalMove,
        )
        .pushes(),
        Actions::from_str(
            "rRRddrruULuu4l3D3u4rdd3L3ruu4ldDldRu6ruLd5luu4rDrd4LDu3ruu4ldDldRu3rddrUru4L3ruu4ldD"
        )
        .unwrap()
        .pushes()
    );
}

#[test]
fn ida_star_search() {
    fn search(mut level: Level, strategy: Strategy) -> Actions {
        let solver = Solver::new(level.map().clone(), strategy);
        let solution = solver.ida_star_search().unwrap();

        let directions = solution.iter().map(|action| action.direction());
        level.execute_batch(directions).unwrap();
        assert!(level.is_solved());

        solution
    }

    search(
        load_level_from_file("assets/BoxWorld_100.xsb", 1),
        Strategy::Fast,
    );
    search(
        load_level_from_file("assets/BoxWorld_100.xsb", 2),
        Strategy::Fast,
    );
    search(
        load_level_from_file("assets/BoxWorld_100.xsb", 3),
        Strategy::Fast,
    );

    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 1),
            Strategy::OptimalPush,
        )
        .pushes(),
        Actions::from_str("DuLLrUUdrR").unwrap().pushes()
    );
    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 1),
            Strategy::OptimalMove,
        )
        .moves(),
        Actions::from_str("DuLLrUUdrR").unwrap().moves()
    );

    assert_eq!(
        search(
            load_level_from_file("assets/BoxWorld_100.xsb", 2),
            Strategy::OptimalPush,
        )
        .pushes(),
        Actions::from_str(
            "rr4DrddlluRdrUl5ulldRur4D3RdrUUd3lddlluRdrUl4ulldRur3D3RdrU3lddlluRdrUlu3R"
        )
        .unwrap()
        .pushes()
    );
    // FIXME:
    // assert_eq!(
    //     search(
    //         load_level_from_file("assets/BoxWorld_100.xsb", 2),
    //         Strategy::OptimalMove,
    //     )
    //     .moves(),
    //     Actions::from_str(
    //         "rr4DrddlluRdrUl5ulldRur4D3RdrUUd3lddlluRdrUl4ulldRur3D3RdrU3lddlluRdrUlu3R"
    //     )
    //     .unwrap()
    //     .moves()
    // );

    // FIXME: Potential infinite loop occurs.
    // assert_eq!(
    //     search(
    //         load_level_from_file("assets/BoxWorld_100.xsb", 3),
    //         Strategy::OptimalPush,
    //     )
    //     .pushes(),
    //     Actions::from_str(
    //         "rRRddrruULuu4l3D3u4rdd3L3ruu4ldDldRu6ruLd5luu4rDrd4LDu3ruu4ldDldRu3rddrUru4L3ruu4ldD"
    //     )
    //     .unwrap()
    //     .pushes()
    // );
    // assert_eq!(
    //     search(
    //         load_level_from_file("assets/BoxWorld_100.xsb", 3),
    //         Strategy::OptimalMove,
    //     )
    //     .pushes(),
    //     Actions::from_str(
    //         "rRRddrruULuu4l3D3u4rdd3L3ruu4ldDldRu6ruLd5luu4rDrd4LDu3ruu4ldDldRu3rddrUru4L3ruu4ldD"
    //     )
    //     .unwrap()
    //     .pushes()
    // );
}

#[expect(dead_code)]
fn print_lower_bounds(solver: &Solver) {
    for y in 0..solver.map().dimensions().y {
        for x in 0..solver.map().dimensions().x {
            let position = Vector2::new(x, y);
            if let Some(lower_bound) = solver.lower_bounds().get(&position) {
                print!("{lower_bound:3} ");
            } else {
                print!("{:3} ", "###");
            }
        }
        println!();
    }
}
