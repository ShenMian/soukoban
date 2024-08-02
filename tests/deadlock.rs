use soukoban::deadlock::*;

mod utils;
use utils::*;

#[test]
fn test_calculate_dead_positions() {
    let map = load_level_from_file("assets/Microban_155.xsb", 3).into_map();
    assert_eq!(calculate_static_deadlocks(&map).len(), 9);

    let map = load_level_from_file("assets/BoxWorld_100.xsb", 9).into_map();
    assert_eq!(calculate_static_deadlocks(&map).len(), 17);
}
