use soukoban::deadlock;

mod utils;
use utils::*;

#[test]
fn calculate_static_deadlocks() {
    let map = load_level_from_file("assets/Microban_155.xsb", 3).into();
    assert_eq!(deadlock::calculate_static_deadlocks(&map).len(), 9);

    let map = load_level_from_file("assets/BoxWorld_100.xsb", 9).into();
    assert_eq!(deadlock::calculate_static_deadlocks(&map).len(), 17);
}
