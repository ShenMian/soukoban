use std::{fs, str::FromStr as _};

use indoc::indoc;
use soukoban::prelude::*;

mod utils;
use utils::*;

#[test]
fn from_str() {
    const SIMPLEST: &str = r"
        #####
        #@$.#
        #####
    ";
    const DUPLICATE_METADATA_LEVEL: &str = r"
        #####
        #@$.#
        #####
        unknown: 1
        unknown: 2
    ";
    const UNTERMINATED_BLOCK_COMMENT_LEVEL: &str = r"
        #####
        #@$.#
        #####
        comment:
        unterminated block comment
    ";
    const INVALID_CHARACTER_LEVEL: &str = r"
        ######
        #@!$.#
        ######
    ";

    assert!(Level::from_str(SIMPLEST).is_ok());
    assert_eq!(
        Level::from_str(DUPLICATE_METADATA_LEVEL).unwrap_err(),
        ParseLevelError::DuplicateMetadata("unknown".to_string())
    );
    assert_eq!(
        Level::from_str(UNTERMINATED_BLOCK_COMMENT_LEVEL).unwrap_err(),
        ParseLevelError::UnterminatedBlockComment
    );
    assert!(matches!(
        Level::from_str(INVALID_CHARACTER_LEVEL).unwrap_err(),
        ParseLevelError::ParseMapError(ParseMapError::InvalidCharacter { ch: '!', .. })
    ));
}

#[test]
fn display() {
    const LEVEL: &str = r"
        ; Level 1
        #####
        #@$.#
        #####
        comment: single line comment
        tile: level title
        comment:
        multi: line
        comment
        comment-end:
        author: level author
    ";
    let level = Level::from_str(LEVEL).unwrap();
    assert_eq!(
        level.to_string(),
        indoc! {"
            #####
            #@$.#
            #####
            author: level author
            comment:
            Level 1
            single line comment
            multi: line
            comment
            comment-end:
            tile: level title
        "}
    );
}

#[test]
fn metadata() {
    const LEVEL: &str = r"
        ; Level 1
        #####
        #@$.#
        #####
        comment: single line comment
        tile: level title
        comment:
        multi
        line
        comment
        comment-end:
        author: level author
    ";
    let level = Level::from_str(LEVEL).unwrap();
    assert_eq!(level.metadata()["tile"], "level title");
    assert_eq!(level.metadata()["author"], "level author");
    assert_eq!(
        level.metadata()["comments"],
        indoc! {"
            Level 1
            single line comment
            multi
            line
            comment
        "}
    );
}

#[test]
fn load_from_str() {
    for entry in fs::read_dir("assets/").unwrap() {
        let path = entry.unwrap().path();
        if path.extension() != Some(std::ffi::OsStr::new("xsb")) {
            continue;
        }
        let count = path
            .to_string_lossy()
            .rsplit_terminator(['_', '.'])
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            Level::load_from_str(&fs::read_to_string(path).unwrap())
                .filter_map(Result::ok)
                .count(),
            count
        );
    }
}

#[test]
fn load_from_reader() {
    for entry in fs::read_dir("assets/").unwrap() {
        let path = entry.unwrap().path();
        if path.extension() != Some(std::ffi::OsStr::new("xsb")) {
            continue;
        }
        let count = path
            .to_string_lossy()
            .rsplit_terminator(['_', '.'])
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let reader = std::io::BufReader::new(fs::File::open(&path).unwrap());
        assert_eq!(
            Level::load_from_reader(reader)
                .filter_map(Result::ok)
                .count(),
            count
        );
    }
}

#[test]
fn load_nth_from_reader() {
    // Microban #3
    const MICROBAN_3_RLE: &str = "--4#|3#--4#|#5-$-#|#-#--#$-#|#-.-.#@-#|9#";
    // Microban II #132
    const MICROBAN2_132_RLE: &str = "18-5#|12-5#-#3-#|12-#3-3#-#-#|6-5#-#-#7-#|5#-#3-#-#3-4#-##|#3-3#-#-#-3#-#--#-#|#-#4-@--#3-#-#--#-3#|#3-4#$6#-4#3-#|3#-#--#-.6-#4-#-#|--#-#--#--##--#4-#3-#|-##-5#--##4-#-5#|-#9-##--3#-#|-#-#-3#-#--5#--#-5#|-#3-#-#4-#-#4-#-#3-#|-5#-#--5#--#-3#-#-#|7-#-3#--##9-#|3-5#-#4-##--5#-##|3-#3-#4-#--##--#--#-#|3-#-#4-#8-#--#-3#|3-#3-4#-6#-4#3-#|3-3#-#--#-#3-#7-#-#|5-#-#--#-3#-#-#-3#3-#|4-##-4#3-#-#3-#-5#|4-#7-#-#-5#|4-#-#-3#3-#|4-#3-#-5#|4-5#";

    assert_eq!(
        Level::from_str(MICROBAN_3_RLE).unwrap(),
        load_level_from_file("assets/Microban_155.xsb", 3)
    );
    assert_eq!(
        Level::from_str(MICROBAN2_132_RLE).unwrap(),
        load_level_from_file("assets/Microban_II_135.xsb", 132)
    );
}

#[test]
fn rotate_cw() {
    const LEVEL: &str = r"
        ###
        #.#
        #$###
        #  @#
        #####
    ";
    let mut level = Level::from_str(LEVEL).unwrap();
    level.rotate_cw();
    assert_eq!(
        level.to_string(),
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
    const LEVEL: &str = r"
        ###
        #.#
        #$###
        #  @#
        #####
    ";
    let mut level = Level::from_str(LEVEL).unwrap();
    level.rotate_ccw();
    assert_eq!(
        level.to_string(),
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
    const LEVEL: &str = r"
        ###
        #.#
        #$###
        #  @#
        #####
    ";
    let mut level = Level::from_str(LEVEL).unwrap();
    level.flip_horizontal();
    assert_eq!(
        level.to_string(),
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
    const LEVEL: &str = r"
        ###
        #.#
        #$###
        #  @#
        #####
    ";
    let mut level = Level::from_str(LEVEL).unwrap();
    level.flip_vertical();
    assert_eq!(
        level.to_string(),
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
fn player_reachable_area() {
    const LEVEL: &str = r"
         #####
         #@* #
         # ###
         # #
         ###
     ";
    let level = Level::from_str(LEVEL).unwrap();
    let actual = level.player_reachable_area();
    let expected = FxHashSet::from_iter([Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)]);
    assert!(actual == expected);

    let unreachable_area = FxHashSet::from_iter([
        Point::new(2, 1),
        Point::new(3, 1),
        Point::new(2, 3),
        Point::new(3, 2),
    ]);
    assert!(actual.is_disjoint(&unreachable_area));
}
