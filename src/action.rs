//! An action.

use std::fmt;

use crate::{direction::Direction, error::ParseActionError};

/// Represents an action.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Action {
    /// Move action in a specified direction.
    Move(Direction),
    /// Push action in a specified direction.
    Push(Direction),
}

impl Action {
    /// Returns the direction associated with the action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::direction::Direction;
    /// use soukoban::Action;
    ///
    /// let action = Action::Move(Direction::Up);
    /// assert_eq!(action.direction(), Direction::Up);
    /// ```
    pub fn direction(&self) -> Direction {
        match *self {
            Action::Move(direction) => direction,
            Action::Push(direction) => direction,
        }
    }

    /// Checks if the action is a move action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::direction::Direction;
    /// use soukoban::Action;
    ///
    /// let action = Action::Move(Direction::Up);
    /// assert!(action.is_move());
    /// ```
    pub fn is_move(&self) -> bool {
        matches!(&self, Action::Move(_))
    }

    /// Checks if the action is a push action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::direction::Direction;
    /// use soukoban::Action;
    ///
    /// let action = Action::Push(Direction::Up);
    /// assert!(action.is_push());
    /// ```
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
