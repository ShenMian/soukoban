use std::{fs, str::FromStr as _};

use indoc::indoc;
use soukoban::prelude::*;

mod utils;
use utils::*;

#[test]
fn from_str() {
    const SIMPLEST: &str = r#"
        #####
        #@$.#
        #####
    "#;
    assert!(Level::from_str(SIMPLEST).is_ok());

    const DUPLICATE_METADATA_LEVEL: &str = r#"
        #####
        #@$.#
        #####
        unknown: 1
        unknown: 2
    "#;
    assert_eq!(
        Level::from_str(DUPLICATE_METADATA_LEVEL).unwrap_err(),
        ParseLevelError::DuplicateMetadata("unknown".to_string())
    );

    const UNTERMINATED_BLOCK_COMMENT_LEVEL: &str = r#"
        #####
        #@$.#
        #####
        comment:
        unterminated block comment
    "#;
    assert_eq!(
        Level::from_str(UNTERMINATED_BLOCK_COMMENT_LEVEL).unwrap_err(),
        ParseLevelError::UnterminatedBlockComment
    );

    const INVALID_CHARACTER_LEVEL: &str = r#"
        ######
        #@!$.#
        ######
    "#;
    assert_eq!(
        Level::from_str(INVALID_CHARACTER_LEVEL).unwrap_err(),
        ParseLevelError::ParseMapError(ParseMapError::InvalidCharacter('!'))
    );
}

#[test]
fn display() {
    let level_str = r#"
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
    "#;
    let level = Level::from_str(level_str).unwrap();
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
    let level_str = r#"
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
    "#;
    let level = Level::from_str(level_str).unwrap();
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
    assert_eq!(
        Level::from_str(MICROBAN_3_RLE).unwrap(),
        load_level_from_file("assets/Microban_155.xsb", 3)
    );

    // Microban II #132
    const MICROBAN2_132_RLE: &str = "18-5#|12-5#-#3-#|12-#3-3#-#-#|6-5#-#-#7-#|5#-#3-#-#3-4#-##|#3-3#-#-#-3#-#--#-#|#-#4-@--#3-#-#--#-3#|#3-4#$6#-4#3-#|3#-#--#-.6-#4-#-#|--#-#--#--##--#4-#3-#|-##-5#--##4-#-5#|-#9-##--3#-#|-#-#-3#-#--5#--#-5#|-#3-#-#4-#-#4-#-#3-#|-5#-#--5#--#-3#-#-#|7-#-3#--##9-#|3-5#-#4-##--5#-##|3-#3-#4-#--##--#--#-#|3-#-#4-#8-#--#-3#|3-#3-4#-6#-4#3-#|3-3#-#--#-#3-#7-#-#|5-#-#--#-3#-#-#-3#3-#|4-##-4#3-#-#3-#-5#|4-#7-#-#-5#|4-#-#-3#3-#|4-#3-#-5#|4-5#";
    assert_eq!(
        Level::from_str(MICROBAN2_132_RLE).unwrap(),
        load_level_from_file("assets/Microban_II_135.xsb", 132)
    );
}

#[test]
fn rotate_cw() {
    let mut level = Level::from_str(
        r#"
        ###
        #.#
        #$###
        #  @#
        #####
    "#,
    )
    .unwrap();
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
    let mut level = Level::from_str(
        r#"
        ###
        #.#
        #$###
        #  @#
        #####
    "#,
    )
    .unwrap();
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
    let mut level = Level::from_str(
        r#"
        ###
        #.#
        #$###
        #  @#
        #####
    "#,
    )
    .unwrap();
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
    let mut level = Level::from_str(
        r#"
        ###
        #.#
        #$###
        #  @#
        #####
    "#,
    )
    .unwrap();
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
    let level = Level::from_str(indoc! {"
        #####
        #@* #
        # ###
        # #
        ###
    "})
    .unwrap();
    let actual = level.player_reachable_area();
    let expected =
        FxHashSet::from_iter([Vector2::new(1, 1), Vector2::new(1, 2), Vector2::new(1, 3)]);
    assert!(actual == expected);

    let unreachable_area = FxHashSet::from_iter([
        Vector2::new(2, 1),
        Vector2::new(3, 1),
        Vector2::new(2, 3),
        Vector2::new(3, 2),
    ]);
    assert!(actual.is_disjoint(&unreachable_area));
}
