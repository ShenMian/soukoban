use std::collections::HashSet;

use nalgebra::Vector2;
use soukoban::path_finding::*;

mod utils;
use utils::*;

#[test]
fn test_find_path() {
    let level = load_level_from_file("assets/Microban II_135.xsb", 132);
    let path = player_move_path(&level, Vector2::new(25, 21)).unwrap();
    assert_eq!(path.len(), 41);
}

#[test]
fn test_box_move_waypoints() {
    let level = load_level_from_file("assets/Microban_155.xsb", 3);
    assert_eq!(box_move_waypoints(&level, Vector2::new(6, 3)).len(), 0);
    let waypoints = box_move_waypoints(&level, Vector2::new(6, 2));
    let positions: HashSet<_> = waypoints.iter().map(|((pos, _), _)| pos).collect();
    assert_eq!(positions.len(), 15);

    let level = load_level_from_file("assets/Microban II_135.xsb", 132);
    let waypoints = box_move_waypoints(&level, Vector2::new(8, 7));
    let positions: HashSet<_> = waypoints.iter().map(|((pos, _), _)| pos).collect();
    let box_path = construct_box_path(Vector2::new(8, 7), Vector2::new(9, 8), &waypoints);
    let player_path = construct_player_path(&level, Vector2::new(7, 6), &box_path);
    assert_eq!(positions.len(), 4 * 35);
    assert_eq!(box_path.len() - 1, 110);
    assert_eq!(player_path.len() - 1, 487);

    let level = load_level_from_file("assets/Microban II_135.xsb", 133);
    let waypoints = box_move_waypoints(&level, Vector2::new(18, 18));
    let positions: HashSet<_> = waypoints.iter().map(|((pos, _), _)| pos).collect();
    let box_path = construct_box_path(Vector2::new(18, 18), Vector2::new(17, 18), &waypoints);
    let player_path = construct_player_path(&level, Vector2::new(16, 18), &box_path);
    assert_eq!(positions.len(), 4 * 6);
    assert_eq!(box_path.len() - 1, 11);
    assert_eq!(player_path.len() - 1, 618);

    let level = load_level_from_file("assets/Microban II_135.xsb", 134);
    let waypoints = box_move_waypoints(&level, Vector2::new(16, 34));
    let box_path = construct_box_path(Vector2::new(16, 34), Vector2::new(20, 34), &waypoints);
    let player_path = construct_player_path(&level, Vector2::new(18, 18), &box_path);
    assert_eq!(box_path.len() - 1, 124);
    assert_eq!(player_path.len() - 1, 5037);
}

#[test]
fn test_pushable_boxes() {
    let level = load_level_from_file("assets/Microban_155.xsb", 3);
    assert_eq!(pushable_boxes(&level), HashSet::from([Vector2::new(6, 2)]));
}
