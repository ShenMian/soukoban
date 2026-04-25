use std::str::FromStr;

use soukoban::prelude::*;

#[test]
fn from_str() {
    assert_eq!(
        Actions::from_str("lUrDL!uRd").unwrap_err(),
        ParseActionsError::ParseActionError(ParseActionError::InvalidCharacter('!'))
    );
    assert_eq!(
        Actions::from_str("!lUrDLuRd").unwrap_err(),
        ParseActionsError::ParseActionError(ParseActionError::InvalidCharacter('!'))
    );
    assert_eq!(
        Actions::from_str("lUrDLuRd!").unwrap_err(),
        ParseActionsError::ParseActionError(ParseActionError::InvalidCharacter('!'))
    );
}

#[test]
fn from_rle_str() {
    assert_eq!(
        Actions::from_str("ruu3LulD4rddlUru3Ldd3luurRDrdL3urDD")
            .unwrap()
            .to_string(),
        "ruuLLLulDrrrrddlUruLLLddllluurRDrdLuuurDD"
    );
    assert_eq!(
        Actions::from_str("ullDLdRuurrdLLrrddlUruL")
            .unwrap()
            .to_string(),
        "ullDLdRuurrdLLrrddlUruL"
    );
}

#[test]
fn rotate_cw() {
    let mut actions = Actions::from_str("lurdLURD").unwrap();
    actions.rotate_cw();
    assert_eq!(actions.to_string(), "urdlURDL");
}

#[test]
fn rotate_ccw() {
    let mut actions = Actions::from_str("lurdLURD").unwrap();
    actions.rotate_ccw();
    assert_eq!(actions.to_string(), "dlurDLUR");
}

#[test]
fn flip_horizontal() {
    let mut actions = Actions::from_str("lurdLURD").unwrap();
    actions.flip_horizontal();
    assert_eq!(actions.to_string(), "ruldRULD");
}

#[test]
fn flip_vertical() {
    let mut actions = Actions::from_str("lurdLURD").unwrap();
    actions.flip_vertical();
    assert_eq!(actions.to_string(), "ldruLDRU");
}

#[test]
fn secondary_values() {
    let empty_actions = Actions::new();
    assert_eq!(empty_actions.moves(), 0);
    assert_eq!(empty_actions.shifts(), 0);
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
    let actions = Actions::from_str("ruuLLLulDrrrrddlUruLLLddllluurRDrdLuuurDD").unwrap();
    assert_eq!(actions.moves(), 41);
    assert_eq!(actions.shifts(), 13);
    let SecondaryValues {
        box_lines,
        box_changes,
        pushing_sessions,
        player_lines: _,
    } = actions.secondary_values();
    assert_eq!(box_lines, 8);
    assert_eq!(box_changes, 4);
    assert_eq!(pushing_sessions, 7);

    // Microban #4
    // ########
    // #      #
    // # .**$@#
    // #      #
    // #####  #
    //     ####
    let actions = Actions::from_str("ullDLdRuurrdLLrrddlUruL").unwrap();
    assert_eq!(actions.moves(), 23);
    assert_eq!(actions.shifts(), 7);
    let SecondaryValues {
        box_lines,
        box_changes,
        pushing_sessions,
        player_lines: _,
    } = actions.secondary_values();
    assert_eq!(box_lines, 6);
    assert_eq!(box_changes, 5);
    assert_eq!(pushing_sessions, 5);

    // Microban #5
    //  #######
    //  #     #
    //  # .$. #
    // ## $@$ #
    // #  .$. #
    // #      #
    // ########
    let actions = Actions::from_str("LulDuu3rdLrrddlUlldRd3luR").unwrap();
    assert_eq!(actions.moves(), 27);
    assert_eq!(actions.shifts(), 6);
    let SecondaryValues {
        box_lines,
        box_changes,
        pushing_sessions,
        player_lines: _,
    } = actions.secondary_values();
    assert_eq!(box_lines, 6);
    assert_eq!(box_changes, 5);
    assert_eq!(pushing_sessions, 6);

    // Microban #6
    // ###### #####
    // #    ###   #
    // # $$     #@#
    // # $ #...   #
    // #   ########
    // #####
    let actions = Actions::from_str(
        "ulld4lulDulld6Rd3ruullDurrddl3Lu5lddrUlu6Rd3ruullDurrddlLLu5lddrUlu5Rd3ruullDurrddlL",
    )
    .unwrap();
    assert_eq!(actions.moves(), 107);
    assert_eq!(actions.shifts(), 29);
    let SecondaryValues {
        box_lines,
        box_changes,
        pushing_sessions,
        player_lines: _,
    } = actions.secondary_values();
    assert_eq!(box_lines, 12);
    assert_eq!(box_changes, 4);
    assert_eq!(pushing_sessions, 12);
}
