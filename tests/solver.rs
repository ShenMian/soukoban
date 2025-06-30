use nalgebra::Vector2;
use soukoban::{solver::*, Level};

mod utils;
use utils::*;

fn solve(mut level: Level) {
    let map = level.map().clone();
    let solver = Solver::new(map, Strategy::Fast);
    let solution = solver.a_star_search().unwrap();
    assert!(solver.ida_star_search().is_ok());
    let directions = solution.iter().map(|action| action.direction());
    level.do_actions(directions).unwrap();
    assert!(level.is_solved());
}

#[test]
fn test_solver() {
    solve(load_level_from_file("assets/BoxWorld_100.xsb", 1));
    solve(load_level_from_file("assets/BoxWorld_100.xsb", 2));
    solve(load_level_from_file("assets/BoxWorld_100.xsb", 3));
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
