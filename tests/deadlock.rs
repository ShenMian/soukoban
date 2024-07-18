use soukoban::deadlock::*;

mod utils;
use utils::*;

#[test]
fn test_calculate_dead_positions() {
    let level = load_level_from_file("assets/Microban_155.xsb", 3);
    assert_eq!(calculate_static_deadlocks(&level).len(), 9);

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 9);
    assert_eq!(calculate_static_deadlocks(&level).len(), 17);
}
