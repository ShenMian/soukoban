use soukoban::direction::Direction;
use soukoban::Level;
use std::str::FromStr;

#[test]
fn can_move_around() {
    use Direction::*;
    let level_str = "
    #####
    # @ #
    # $ #
    #.  #
    #####";

    let mut level = Level::from_str(level_str).unwrap();

    for direction in vec![Right, Down, Down, Up, Up] {
        level.do_action(direction).unwrap()
    }
}
