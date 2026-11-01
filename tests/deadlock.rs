use indoc::indoc;
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
fn is_freeze_deadlock_detects_mixed_frozen_groups() {
    let map: Map = indoc! {"
        ######
        #*$  #
        #  @ #
        #.   #
        ######
    "}
    .parse()
    .unwrap();
    assert!(deadlock::is_freeze_deadlock(
        &map,
        Point::new(1, 1),
        map.box_positions(),
    ));
    assert!(deadlock::is_freeze_deadlock(
        &map,
        Point::new(2, 1),
        map.box_positions(),
    ));

    let map: Map = indoc! {"
        ######
        #**  #
        #  @ #
        #    #
        ######
    "}
    .parse()
    .unwrap();

    assert!(!deadlock::is_freeze_deadlock(
        &map,
        Point::new(1, 1),
        map.box_positions(),
    ));
    assert!(!deadlock::is_freeze_deadlock(
        &map,
        Point::new(2, 1),
        map.box_positions(),
    ));
}

#[test]
fn compute_dead_positions_detects_non_corner_dead_positions() {
    let map: Map = indoc! {"
        ######
        #  # #
        #  $ #
        ##@ ##
        #.  ##
        ######
    "}
    .parse()
    .unwrap();

    let dead_positions = deadlock::compute_dead_positions(&map);
    assert!(dead_positions.contains(&Point::new(3, 2)));
    assert!(dead_positions.contains(&Point::new(3, 3)));
}
