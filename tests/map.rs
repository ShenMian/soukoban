use std::str::FromStr as _;

use indoc::indoc;
use soukoban::prelude::*;

mod utils;
use utils::*;

#[test]
fn from_str() {
    const NO_PLAYER_MAP: &str = r"
        #####
        # $.#
        #####
    ";
    const NO_BOX_OR_GOAL_MAP: &str = r"
        ###
        #@#
        ###
    ";
    const MORE_THAN_ONE_PLAYER_MAP_1: &str = r"
        ######
        #@@$.#
        ######
    ";
    const MORE_THAN_ONE_PLAYER_MAP_2: &str = r"
        ######
        #@$.+#
        ######
    ";
    const MISMATCH_BETWEEN_BOXS_AND_GOALS_MAP: &str = r"
        ######
        #@$$.#
        ######
    ";
    const INVALID_CHARACTER_MAP: &str = r"
        ######
        #@!$.#
        ######
    ";

    assert_eq!(
        Map::from_str(NO_PLAYER_MAP).unwrap_err(),
        ParseMapError::MissingPlayer
    );
    assert_eq!(
        Map::from_str(NO_BOX_OR_GOAL_MAP).unwrap_err(),
        ParseMapError::MissingBoxOrGoal
    );
    assert_eq!(
        Map::from_str(MORE_THAN_ONE_PLAYER_MAP_1).unwrap_err(),
        ParseMapError::MultiplePlayers
    );
    assert_eq!(
        Map::from_str(MORE_THAN_ONE_PLAYER_MAP_2).unwrap_err(),
        ParseMapError::MultiplePlayers
    );
    assert_eq!(
        Map::from_str(MISMATCH_BETWEEN_BOXS_AND_GOALS_MAP).unwrap_err(),
        ParseMapError::BoxGoalMismatch
    );
    assert_eq!(
        Map::from_str(INVALID_CHARACTER_MAP).unwrap_err(),
        ParseMapError::InvalidCharacter('!')
    );
}

#[test]
fn from_actions() {
    assert!(Map::from_actions(Actions::from_str("R").unwrap()).is_ok());
    assert!(Map::from_actions(Actions::from_str("DuLLrUUdrR").unwrap()).is_ok());

    assert_eq!(
        Map::from_actions(Actions::from_str("RddrU").unwrap()).unwrap_err(),
        ParseMapError::InvalidActions
    );
    assert_eq!(
        Map::from_actions(Actions::from_str("RdU").unwrap()).unwrap_err(),
        ParseMapError::InvalidActions
    );
    assert_eq!(
        Map::from_actions(Actions::from_str("RL").unwrap()).unwrap_err(),
        ParseMapError::InvalidActions
    );
    assert_eq!(
        Map::from_actions(Actions::from_str("llurldd").unwrap()).unwrap_err(),
        ParseMapError::MissingBoxOrGoal
    );

    let actions =
        Actions::from_str("uulLdlluRRllddlluuRRdrruRurDDulldldddllUdrruuluullddRluurrdrrurrdDldLrurrdLLuruulldlluRRRurDDullllllddrddrrUUddlluuluurrdRurrrdDldLrurrdLLuruullllllddrddrrUULuurrrrdddlLruruullllddrUluRRRurDDullllllddRddrrUUdrrrruLdllluUluRRRurDDDrdLL")
            .unwrap();
    assert_eq!(
        Map::from_actions(actions).unwrap(),
        Map::from_str(
            r"
            -----####-
            ######  #-
            # $  $  #-
            # #  .# ##
            #  . #.@ #
            ##$# *   #
            -#   #####
            -#####----
        "
        )
        .unwrap()
    );
}

#[test]
fn get() {
    let mut map: Map = load_level_from_file("assets/Holland_81.xsb", 9).into();
    for x in 0..map.dimensions().x {
        for y in 0..map.dimensions().y {
            let position = Point::new(x, y);
            let tiles = map[position];
            assert_eq!(tiles, *map.get(position).unwrap());
            assert_eq!(tiles, unsafe { *map.get_unchecked(position) });
            assert_eq!(tiles, *map.get_mut(position).unwrap());
            assert_eq!(tiles, unsafe { *map.get_unchecked_mut(position) });
        }
    }
}

#[test]
fn canonicalize() {
    // Steaming Hot
    let mut actual = Map::from_str(
        r"
         #      #
         #   #  #
          # #  #
           # #  #
          #   #  #
         #   #  #
          # #  #
        -
        ##########
        #........####
        # $$$$$$$#  #
        #.$......# *#
        # $$$$$$ #  #
        #......$+# *#
        #$$$$$$$ #  #
        #        ####
        ##########
    ",
    )
    .unwrap();
    let expected = Map::from_str(
        r"
        ##########
        #@_______#
        #_$$$$$$$#
        #.$......#
        #_$$$$$$_#
        #......$.#
        #$$$$$$$_#
        #........#
        ##########
    ",
    )
    .unwrap();
    actual.canonicalize();
    assert_eq!(actual, expected);

    // Sasquatch #41
    let mut actual = load_level_from_file("assets/Sasquatch_50.xsb", 41)
        .map()
        .clone();
    let expected = Map::from_str(
        r"
        --####----
        --#@_##---
        --#___#---
        --#___#---
        ###$_$####
        #_$...$__#
        #__._.___#
        #_$...$__#
        ###$_$####
        --#___#---
        --#####---
    ",
    )
    .unwrap();
    actual.canonicalize();
    assert_eq!(actual, expected);

    let mut actual = load_level_from_file("assets/Benchmark_3.xsb", 3)
        .map()
        .clone();
    let expected = Map::from_str(
        r"
        ----#####--------
        --###___##-------
        ###___*._####----
        #__**__*_#__#----
        #_*__**#____###--
        #__*______#___#--
        #_****$_####__###
        #*____*_#--##___#
        #_****_##---##__#
        #_______#----####
        #.*****_#--------
        #______##--------
        ##_$**__#--------
        -#_*@_*_#--------
        -###***##--------
        ---#___#---------
        ---#####---------
    ",
    )
    .unwrap();
    actual.canonicalize();
    assert_eq!(actual, expected);
}

#[test]
fn shrink_to_fit() {
    let mut oversize_map = Map::from_str(
        r"
        ---------------
        ---------------
        ----####-------
        --###  ####----
        --#     $ #----
        --# #  #$ #----
        --# . .#@ #----
        --#########----
        ---------------
        ---------------
        ---------------
    ",
    )
    .unwrap();
    let expected = load_level_from_file("assets/Microban_155.xsb", 3)
        .map()
        .clone();
    oversize_map.shrink_to_fit();
    assert_eq!(oversize_map, expected);

    let mut actual = expected.clone();
    actual.shrink_to_fit();
    assert_eq!(actual, expected);
}

#[test]
fn rotate_cw() {
    let mut map = Map::from_str(
        r"
        ###
        #.#
        #$###
        #  @#
        #####
    ",
    )
    .unwrap();
    map.rotate_cw();
    assert_eq!(
        map.to_string(),
        indoc! {"
            #####
            #_$.#
            #_###
            #@#--
            ###--
        "}
    );
}

#[test]
fn rotate_ccw() {
    let mut map = Map::from_str(
        r"
        ###
        #.#
        #$###
        #  @#
        #####
    ",
    )
    .unwrap();
    map.rotate_ccw();
    assert_eq!(
        map.to_string(),
        indoc! {"
            --###
            --#@#
            ###_#
            #.$_#
            #####
        "}
    );
}

#[test]
fn flip_horizontal() {
    let mut map = Map::from_str(
        r"
        ###
        #.#
        #$###
        #  @#
        #####
    ",
    )
    .unwrap();
    map.flip_horizontal();
    assert_eq!(
        map.to_string(),
        indoc! {"
            --###
            --#.#
            ###$#
            #@__#
            #####
        "}
    );
}

#[test]
fn flip_vertical() {
    let mut map = Map::from_str(
        r"
        ###
        #.#
        #$###
        #  @#
        #####
    ",
    )
    .unwrap();
    map.flip_vertical();
    assert_eq!(
        map.to_string(),
        indoc! {"
            #####
            #__@#
            #$###
            #.#--
            ###--
        "}
    );
}

#[test]
fn display() {
    let map = load_level_from_file("assets/Holland_81.xsb", 9)
        .map()
        .clone();
    assert_eq!(
        map.to_string(),
        indoc! {"
            --####--
            -#____#-
            -#._*_#-
            #_._$__#
            #_#**#_#
            #__*+*_#
            -#_$$_#-
            -#____#-
            --####--
        "}
    );

    let mut map = load_level_from_file("assets/Holland_81.xsb", 9)
        .map()
        .clone();
    map[Point::new(4, 2)].insert(Tiles::Player);
    assert_eq!(
        map.to_string(),
        indoc! {"
            --####--
            -#____#-
            -#._?_#-
            #_._$__#
            #_#**#_#
            #__*+*_#
            -#_$$_#-
            -#____#-
            --####--
        "}
    );
}
