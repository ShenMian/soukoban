use soukoban::{deadlock, prelude::*};

mod utils;
use utils::*;

#[test]
fn is_freeze_deadlock() {
    let mut level = load_level_from_file("assets/Deadlock_4.xsb", 2);
    level.execute(Direction::Up).unwrap();
    assert!(deadlock::is_freeze_deadlock(
        level.map(),
        Point::new(3, 1),
        level.map().box_positions(),
    ));

    let mut level = load_level_from_file("assets/Deadlock_4.xsb", 3);
    level.execute(Direction::Left).unwrap();
    assert!(deadlock::is_freeze_deadlock(
        level.map(),
        Point::new(4, 2),
        level.map().box_positions(),
    ));

    let mut level = load_level_from_file("assets/Deadlock_4.xsb", 4);
    level.execute(Direction::Left).unwrap();
    assert!(level.is_solved());
    assert!(!deadlock::is_freeze_deadlock(
        level.map(),
        Point::new(4, 2),
        level.map().box_positions(),
    ));
}

#[test]
fn compute_static_deadlocks() {
    let map: Map = load_level_from_file("assets/Deadlock_4.xsb", 1).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 9);

    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 9);

    let map = load_level_from_file("assets/BoxWorld_100.xsb", 9).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 17);
}
