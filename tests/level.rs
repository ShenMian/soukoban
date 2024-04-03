use std::{fs, str::FromStr};

use indoc::indoc;
use soukoban::{Level, ParseLevelError, ParseMapError};

mod utilies;
use utilies::*;

#[test]
fn parse_level_error() {
    let duplicate_metadata_level = r#"
        #####
        #@$.#
        #####
        unknown: 1
        unknown: 2
    "#;
    let unterminated_block_comment_level = r#"
        #####
        #@$.#
        #####
        comment:
        unterminated block comment
    "#;
    assert!(Level::from_str(SIMPLEST).is_ok());
    assert_eq!(
        Level::from_str(duplicate_metadata_level).unwrap_err(),
        ParseLevelError::DuplicateMetadata("unknown".to_string())
    );
    assert_eq!(
        Level::from_str(unterminated_block_comment_level).unwrap_err(),
        ParseLevelError::UnterminatedBlockComment
    );

    let invalid_character_level = r#"
        ######
        #@!$.#
        ######
    "#;
    assert_eq!(
        Level::from_str(invalid_character_level).unwrap_err(),
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
fn create_multiple_levels_with_xsb() {
    for entry in fs::read_dir("assets/").unwrap() {
        let path = entry.unwrap().path();
        if path.extension() != Some(std::ffi::OsStr::new(".xsb")) {
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
            Level::load(&fs::read_to_string(path).unwrap())
                .filter_map(Result::ok)
                .count(),
            count
        );
    }
}

#[test]
fn create_level_with_rle_xsb() {
    assert_eq!(
        Level::from_str(MICROBAN_3_RLE).unwrap(),
        load_level_from_file("assets/Microban_155.xsb", 3)
    );
    assert_eq!(
        Level::from_str(MICROBAN2_132_RLE).unwrap(),
        load_level_from_file("assets/Microban II_135.xsb", 132)
    );
}

// Simplest level
const SIMPLEST: &str = r#"
    #####
    #@$.#
    #####
"#;

// Microban #3
const MICROBAN_3_RLE: &str = "--4#|3#--4#|#5-$-#|#-#--#$-#|#-.-.#@-#|9#";

// Microban II #132
const MICROBAN2_132_RLE: &str = "18-5#|12-5#-#3-#|12-#3-3#-#-#|6-5#-#-#7-#|5#-#3-#-#3-4#-##|#3-3#-#-#-3#-#--#-#|#-#4-@--#3-#-#--#-3#|#3-4#$6#-4#3-#|3#-#--#-.6-#4-#-#|--#-#--#--##--#4-#3-#|-##-5#--##4-#-5#|-#9-##--3#-#|-#-#-3#-#--5#--#-5#|-#3-#-#4-#-#4-#-#3-#|-5#-#--5#--#-3#-#-#|7-#-3#--##9-#|3-5#-#4-##--5#-##|3-#3-#4-#--##--#--#-#|3-#-#4-#8-#--#-3#|3-#3-4#-6#-4#3-#|3-3#-#--#-#3-#7-#-#|5-#-#--#-3#-#-#-3#3-#|4-##-4#3-#-#3-#-5#|4-#7-#-#-5#|4-#-#-3#3-#|4-#3-#-5#|4-5#";
