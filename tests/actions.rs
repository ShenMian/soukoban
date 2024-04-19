use std::str::FromStr;

use soukoban::{Actions, ParseActionError, SecondaryValues};

#[test]
fn parse_actions_error() {
    assert_eq!(
        Actions::from_str("lUrDL!uRd").unwrap_err(),
        ParseActionError::InvalidCharacter('!')
    );
}

#[test]
fn rle_decode() {
    assert_eq!(
        Actions::from_str("ruu4L4rddlUru3LulDrdd3luuRRDrdL3urDD")
            .unwrap()
            .to_string(),
        "ruuLLLLrrrrddlUruLLLulDrddllluuRRDrdLuuurDD"
    );
    assert_eq!(
        Actions::from_str("ullDullddrRuLu3rdLLrrddlUruL")
            .unwrap()
            .to_string(),
        "ullDullddrRuLurrrdLLrrddlUruL"
    );
}

#[test]
fn scoring_metrics() {
    let empty_actions = Actions::from_str("").unwrap();
    assert_eq!(empty_actions.moves(), 0);
    assert_eq!(empty_actions.pushes(), 0);
    let SecondaryValues {
        box_lines,
        box_changes,
        pushing_sessions,
        player_lines,
    } = empty_actions.secondary_values();
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
    let actions = Actions::from_str("ruuLLLLrrrrddlUruLLLulDrddllluuRRDrdLuuurDD").unwrap();
    assert_eq!(actions.moves(), 43);
    assert_eq!(actions.pushes(), 15);
    let SecondaryValues {
        box_lines,
        box_changes,
        pushing_sessions,
        player_lines,
    } = actions.secondary_values();
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
    let actions = Actions::from_str("ullDullddrRuLurrrdLLrrddlUruL").unwrap();
    let SecondaryValues {
        box_lines,
        box_changes,
        pushing_sessions,
        player_lines,
    } = actions.secondary_values();
    assert_eq!(box_lines, 6);
    assert_eq!(box_changes, 4);
    assert_eq!(pushing_sessions, 6);
    assert_eq!(player_lines, 20);
}
