use soukoban::{deadlock, prelude::*};

mod utils;
use utils::*;

#[test]
fn is_freeze_deadlock() {
    let mut map: Map = load_level_from_file("assets/Deadlock_3.xsb", 2).into();
    map.set_box_position(Vector2::new(3, 2), Vector2::new(3, 1));
    assert!(deadlock::is_freeze_deadlock(
        &map,
        Vector2::new(3, 1),
        map.box_positions(),
    ));

    let mut map: Map = load_level_from_file("assets/Deadlock_3.xsb", 3).into();
    map.set_box_position(Vector2::new(5, 2), Vector2::new(4, 2));
    assert!(deadlock::is_freeze_deadlock(
        &map,
        Vector2::new(4, 2),
        map.box_positions(),
    ));
}

#[test]
fn compute_static_deadlocks() {
    let map: Map = load_level_from_file("assets/Deadlock_3.xsb", 1).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 9);

    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 9);

    let map = load_level_from_file("assets/BoxWorld_100.xsb", 9).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 17);
}
