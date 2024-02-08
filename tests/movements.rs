use soukoban::{Actions, ParseActionError};

#[test]
fn parse_movement_error() {
    assert_eq!(
        Actions::with_str("lUrDL!uRd").unwrap_err(),
        ParseActionError::InvalidCharacter('!')
    );
}

#[test]
fn rle_decode() {
    assert_eq!(
        Actions::with_str("ruu4L4rddlUru3LulDrdd3luuRRDrdL3urDD")
            .unwrap()
            .to_string(),
        "ruuLLLLrrrrddlUruLLLulDrddllluuRRDrdLuuurDD"
    );
    assert_eq!(
        Actions::with_str("ullDullddrRuLu3rdLLrrddlUruL")
            .unwrap()
            .to_string(),
        "ullDullddrRuLurrrdLLrrddlUruL"
    );
}

#[test]
fn scoring_metrics() {
    let empty_movements = Actions::with_str("").unwrap();
    assert_eq!(empty_movements.moves(), 0);
    assert_eq!(empty_movements.pushes(), 0);
    let (box_lines, box_changes, pushing_sessions, player_lines) =
        empty_movements.secondary_values();
    assert_eq!(box_lines, 0);
    assert_eq!(box_changes, 0);
    assert_eq!(pushing_sessions, 0);
    assert_eq!(player_lines, 0);

    // Microban #3
    //   ####
    // ###  ####
    // #     $ #
    // # #  #$ #
    // # . .#@ #
    // #########
    // box lines     : 8
    // pushing sessions: 7
    let movements = Actions::with_str("ruuLLLLrrrrddlUruLLLulDrddllluuRRDrdLuuurDD").unwrap();
    assert_eq!(movements.moves(), 43);
    assert_eq!(movements.pushes(), 15);
    let (box_lines, box_changes, pushing_sessions, player_lines) = movements.secondary_values();
    assert_eq!(box_lines, 8);
    assert_eq!(box_changes, 5);
    assert_eq!(pushing_sessions, 7);
    assert_eq!(player_lines, 25);

    // Microban #4
    // ########
    // #      #
    // # .**$@#
    // #      #
    // #####  #
    //     ####
    // box lines     : 6
    // pushing sessions: 6
    let movements = Actions::with_str("ullDullddrRuLurrrdLLrrddlUruL").unwrap();
    let (box_lines, box_changes, pushing_sessions, player_lines) = movements.secondary_values();
    assert_eq!(box_lines, 6);
    assert_eq!(box_changes, 4);
    assert_eq!(pushing_sessions, 6);
    assert_eq!(player_lines, 20);
}
