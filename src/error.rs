//! Error types.

use thiserror::Error;

/// An error which can be returned when parsing a level.
#[derive(Error, Clone, Eq, PartialEq, Debug)]
pub enum ParseLevelError {
    /// There is a duplicate metadata with the same key.
    #[error("duplicate metadata with key `{0}`")]
    DuplicateMetadata(String),
    /// There is an unterminated block comment.
    #[error("unterminated block comment")]
    UnterminatedBlockComment,
    /// There is no map data.
    #[error("no map data")]
    NoMap,
    /// There is an error when parsing the map.
    #[error(transparent)]
    ParseMapError(#[from] ParseMapError),
}

/// An error which can be returned when parsing a map.
#[derive(Error, Clone, Eq, PartialEq, Debug)]
pub enum ParseMapError {
    /// There are no players. There should be only one player.
    #[error("no player")]
    NoPlayer,
    /// There are multiple players. There should only be one player.
    #[error("more than one player")]
    MoreThanOnePlayer,
    /// The number of boxes and targets are inconsistent. They should be
    /// consistent.
    #[error("mismatch between number of boxes and goals")]
    BoxGoalMismatch,
    /// There are no boxes or goals. There should be at least one box and one
    /// goal.
    #[error("no box or goal")]
    NoBoxOrGoal,
    /// Contains non-XSB format character.
    #[error("invalid character: `{0}`")]
    InvalidCharacter(char),
    /// Actions are invalid and cannot be used to build and create levels.
    #[error("invalid actions")]
    InvalidActions,
}

/// An error which can be returned when parsing a action.
#[derive(Error, Clone, Eq, PartialEq, Debug)]
pub enum ParseActionError {
    /// Contains non-LURD format character.
    #[error("invalid character: `{0}`")]
    InvalidCharacter(char),
}

/// An error which can be returned when encoding RLE.
#[derive(Error, Clone, Eq, PartialEq, Debug)]
pub enum EncodeRleError {
    /// Unencodable characters encountered, such as numbers.
    #[error("invalid character: `{0}`")]
    InvalidCharacter(char),
}

/// An error which can be returned when decoding RLE.
#[derive(Error, Clone, Eq, PartialEq, Debug)]
pub enum DecodeRleError {
    /// The decoded string ends with digits.
    #[error("end with digits: `{0}`")]
    EndWithDigits(usize),
}

/// An error which can be returned when searching for a solution.
#[derive(Error, Clone, Eq, PartialEq, Debug)]
pub enum SearchError {
    /// No solution found.
    #[error("no solution found")]
    NoSolution,
}

/// An error which can be returned when level do/undo/redo actions.
#[derive(Error, Clone, Eq, PartialEq, Debug)]
pub enum ActionError {
    /// Movement in the specified direction is blocked.
    #[error("movement in the specified direction is blocked")]
    MoveBlocked,
    /// Push in the specified direction is blocked.
    #[error("push in the specified direction is blocked")]
    PushBlocked,
    /// No actions.
    #[error("no actions")]
    NoActions,
    /// No undone actions.
    #[error("no undone actions")]
    NoUndoneActions,
}
