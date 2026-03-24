use std::collections::HashSet;

use nalgebra::Vector2;
use soukoban::{Map, deadlock};

mod utils;
use utils::*;

#[test]
fn static_deadlocks() {
    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 9);

    let map = load_level_from_file("assets/BoxWorld_100.xsb", 9).into();
    assert_eq!(deadlock::compute_static_deadlocks(&map).len(), 17);
}

#[test]
fn freeze_deadlocks() {
    let mut freeze_1: Map = load_level_from_file("assets/Deadlock_2.xsb", 1).into();
    freeze_1.set_box_position(Vector2::new(3, 2), Vector2::new(3, 1));
    assert!(deadlock::is_freeze_deadlock(
        &freeze_1,
        Vector2::new(3, 1),
        freeze_1.box_positions(),
        &mut HashSet::new(),
    ));

    let mut freeze_2: Map = load_level_from_file("assets/Deadlock_2.xsb", 2).into();
    freeze_2.set_box_position(Vector2::new(5, 2), Vector2::new(4, 2));
    assert!(deadlock::is_freeze_deadlock(
        &freeze_2,
        Vector2::new(4, 2),
        freeze_2.box_positions(),
        &mut HashSet::new(),
    ));
}
