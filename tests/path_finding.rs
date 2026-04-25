use nalgebra::Vector2;
use soukoban::{FxHashSet, direction::*, path_finding::*, solver::Strategy};

mod utils;
use utils::*;

#[test]
fn find_path() {
    let map = load_level_from_file("assets/Microban_II_135.xsb", 132).into();
    let path = compute_player_move_directions(&map, Vector2::new(25, 21)).unwrap();
    assert_eq!(path.len(), 41);
}

#[test]
fn test_compute_box_waypoints() {
    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    let (waypoints, _) = compute_box_waypoints(&map, Vector2::new(6, 2), Strategy::PushOptimal);
    let positions: FxHashSet<_> = waypoints
        .keys()
        .map(
            |DirectedPosition {
                 position,
                 direction: _,
             }| position,
        )
        .collect();
    assert_eq!(positions.len(), 15);

    let map = load_level_from_file("assets/Microban_II_135.xsb", 132).into();
    let (waypoints, _) = compute_box_waypoints(&map, Vector2::new(8, 7), Strategy::PushOptimal);
    let positions: FxHashSet<_> = waypoints
        .keys()
        .map(
            |DirectedPosition {
                 position,
                 direction: _,
             }| position,
        )
        .collect();
    let box_path = construct_box_path(
        DirectedPosition::new(Vector2::new(9, 8), Direction::Left),
        &waypoints,
    );
    let player_path = construct_player_path(&map, Vector2::new(7, 6), &box_path);
    assert_eq!(positions.len(), 4 * 35);
    assert_eq!(box_path.len() - 1, 110);
    assert_eq!(player_path.len() - 1, 487);

    let map = load_level_from_file("assets/Microban_II_135.xsb", 133).into();
    let (waypoints, _) = compute_box_waypoints(&map, Vector2::new(18, 18), Strategy::PushOptimal);
    let positions: FxHashSet<_> = waypoints
        .keys()
        .map(
            |DirectedPosition {
                 position,
                 direction: _,
             }| position,
        )
        .collect();
    let box_path = construct_box_path(
        DirectedPosition::new(Vector2::new(17, 18), Direction::Right),
        &waypoints,
    );
    let player_path = construct_player_path(&map, Vector2::new(16, 18), &box_path);
    assert_eq!(positions.len(), 4 * 6);
    assert_eq!(box_path.len() - 1, 11);
    assert_eq!(player_path.len() - 1, 618);

    let map = load_level_from_file("assets/Microban_II_135.xsb", 134).into();
    let (waypoints, _) = compute_box_waypoints(&map, Vector2::new(16, 34), Strategy::PushOptimal);
    let box_path = construct_box_path(
        DirectedPosition::new(Vector2::new(20, 34), Direction::Left),
        &waypoints,
    );
    let player_path = construct_player_path(&map, Vector2::new(18, 18), &box_path);
    assert_eq!(box_path.len() - 1, 124);
    assert_eq!(player_path.len() - 1, 5037);

    let map = load_level_from_file("assets/Microban_II_135.xsb", 135).into();
    let (waypoints, _) = compute_box_waypoints(&map, Vector2::new(21, 36), Strategy::PushOptimal);
    let box_path = construct_box_path(
        DirectedPosition::new(Vector2::new(21, 37), Direction::Down),
        &waypoints,
    );
    assert_eq!(box_path.len() - 1, 591);
    let player_path = construct_player_path(&map, Vector2::new(21, 38), &box_path);
    assert_eq!(player_path.len() - 1, 1108);
}

#[test]
fn test_pushable_boxes() {
    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    assert_eq!(
        compute_pushable_boxes(&map),
        FxHashSet::from_iter([Vector2::new(6, 2)])
    );
}
