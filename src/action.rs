use std::fmt;

use crate::{direction::Direction, error::ParseActionError};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Action {
    Move(Direction),
    Push(Direction),
}

impl Action {
    pub fn direction(&self) -> Direction {
        match *self {
            Action::Move(direction) => direction,
            Action::Push(direction) => direction,
        }
    }

    pub fn is_move(&self) -> bool {
        matches!(&self, Action::Move(_))
    }

    pub fn is_push(&self) -> bool {
        matches!(&self, Action::Push(_))
    }
}

impl TryFrom<char> for Action {
    type Error = ParseActionError;

    fn try_from(char: char) -> Result<Self, ParseActionError> {
        let direction = match char.to_ascii_lowercase() {
            'u' => Direction::Up,
            'd' => Direction::Down,
            'l' => Direction::Left,
            'r' => Direction::Right,
            _ => return Err(ParseActionError::InvalidCharacter(char)),
        };
        if char.is_ascii_uppercase() {
            Ok(Action::Push(direction))
        } else {
            Ok(Action::Move(direction))
        }
    }
}

impl From<Action> for char {
    fn from(action: Action) -> Self {
        let char = match action.direction() {
            Direction::Up => 'u',
            Direction::Down => 'd',
            Direction::Left => 'l',
            Direction::Right => 'r',
        };
        if action.is_push() {
            char.to_ascii_uppercase()
        } else {
            char
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<char>::into(*self))
    }
}
