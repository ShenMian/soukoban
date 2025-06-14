use std::collections::HashSet;

use soukoban::path_finding::*;
use soukoban::point::Point;

mod utils;
use utils::*;

#[test]
fn find_path() {
    let map = load_level_from_file("assets/Microban II_135.xsb", 132).into();
    let path = player_move_path(&map, Point::new(25, 21)).unwrap();
    assert_eq!(path.len(), 41);
}

#[test]
fn test_box_move_waypoints() {
    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    assert_eq!(box_move_waypoints(&map, Point::new(6, 3)).len(), 0);
    let waypoints = box_move_waypoints(&map, Point::new(6, 2));
    let positions: HashSet<_> = waypoints.iter().map(|((pos, _), _)| pos).collect();
    assert_eq!(positions.len(), 15);

    let map = load_level_from_file("assets/Microban II_135.xsb", 132).into();
    let waypoints = box_move_waypoints(&map, Point::new(8, 7));
    let positions: HashSet<_> = waypoints.iter().map(|((pos, _), _)| pos).collect();
    let box_path = construct_box_path(Point::new(8, 7), Point::new(9, 8), &waypoints);
    let player_path = construct_player_path(&map, Point::new(7, 6), &box_path);
    assert_eq!(positions.len(), 4 * 35);
    assert_eq!(box_path.len() - 1, 110);
    assert_eq!(player_path.len() - 1, 487);

    let map = load_level_from_file("assets/Microban II_135.xsb", 133).into();
    let waypoints = box_move_waypoints(&map, Point::new(18, 18));
    let positions: HashSet<_> = waypoints.iter().map(|((pos, _), _)| pos).collect();
    let box_path = construct_box_path(Point::new(18, 18), Point::new(17, 18), &waypoints);
    let player_path = construct_player_path(&map, Point::new(16, 18), &box_path);
    assert_eq!(positions.len(), 4 * 6);
    assert_eq!(box_path.len() - 1, 11);
    assert_eq!(player_path.len() - 1, 618);

    let map = load_level_from_file("assets/Microban II_135.xsb", 134).into();
    let waypoints = box_move_waypoints(&map, Point::new(16, 34));
    let box_path = construct_box_path(Point::new(16, 34), Point::new(20, 34), &waypoints);
    let player_path = construct_player_path(&map, Point::new(18, 18), &box_path);
    assert_eq!(box_path.len() - 1, 124);
    assert_eq!(player_path.len() - 1, 5037);

    // FIXME:
    // let map = load_level_from_file("assets/Microban II_135.xsb", 135).into();
    // let waypoints = box_move_waypoints(&map, Vector2::new(21, 36));
    // let box_path = construct_box_path(Vector2::new(21, 36), Vector2::new(21,
    // 37), &waypoints); assert_eq!(box_path.len() - 1, 591);
    // let player_path = construct_player_path(&map, Vector2::new(21, 38),
    // &box_path); assert_eq!(player_path.len() - 1, 1108);
}

#[test]
fn test_pushable_boxes() {
    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    assert_eq!(pushable_boxes(&map), HashSet::from([Point::new(6, 2)]));
}
