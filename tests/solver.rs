use nalgebra::Vector2;
use soukoban::solver::*;

mod utils;
use utils::*;

#[test]
fn test_solver() {
    let map = load_level_from_file("assets/BoxWorld_100.xsb", 1)
        .map()
        .clone();
    let solver = Solver::new(map.clone(), Strategy::Fast);
    assert!(solver.a_star_search(&map).is_ok());
    assert!(solver.ida_star_search(&map).is_ok());

    let map = load_level_from_file("assets/BoxWorld_100.xsb", 2)
        .map()
        .clone();
    let solver = Solver::new(map.clone(), Strategy::Fast);
    assert!(solver.a_star_search(&map).is_ok());
    assert!(solver.ida_star_search(&map).is_ok());

    let map = load_level_from_file("assets/BoxWorld_100.xsb", 3)
        .map()
        .clone();
    let solver = Solver::new(map.clone(), Strategy::Fast);
    assert!(solver.a_star_search(&map).is_ok());
    assert!(solver.ida_star_search(&map).is_ok());
}

#[allow(dead_code)]
fn print_lower_bounds(solver: &Solver) {
    for y in 0..solver.map().dimensions().y {
        for x in 0..solver.map().dimensions().x {
            let position = Vector2::new(x, y);
            if let Some(lower_bound) = solver.lower_bounds().get(&position) {
                print!("{:3} ", lower_bound);
            } else {
                print!("{:3} ", "###");
            }
        }
        println!();
    }
}
