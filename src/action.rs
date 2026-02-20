//! An action.

use std::fmt;

use crate::{direction::Direction, error::ParseActionError};

/// Represents an action.
pub trait Action {
    /// Returns the direction associated with the action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::direction::Direction;
    /// use soukoban::{Action, ForwardAction};
    ///
    /// let action = ForwardAction::Move(Direction::Up);
    /// assert_eq!(action.direction(), Direction::Up);
    /// ```
    fn direction(&self) -> Direction;

    /// Checks if the action is a move action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::direction::Direction;
    /// use soukoban::{Action, ForwardAction};
    ///
    /// let action = ForwardAction::Move(Direction::Up);
    /// assert!(action.is_move());
    /// ```
    fn is_move(&self) -> bool;
}

/// Represents a forward action.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ForwardAction {
    /// Move action in a specified direction.
    Move(Direction),
    /// Push action in a specified direction.
    Push(Direction),
}

impl ForwardAction {
    /// Checks if the action is a push action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::ForwardAction;
    /// use soukoban::direction::Direction;
    ///
    /// let action = ForwardAction::Push(Direction::Up);
    /// assert!(action.is_push());
    /// ```
    pub fn is_push(&self) -> bool {
        matches!(&self, ForwardAction::Push(_))
    }
}

impl Action for ForwardAction {
    fn direction(&self) -> Direction {
        match *self {
            ForwardAction::Move(direction) => direction,
            ForwardAction::Push(direction) => direction,
        }
    }

    fn is_move(&self) -> bool {
        matches!(&self, ForwardAction::Move(_))
    }
}

/// Represents a reverse action.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ReverseAction {
    /// Move action in a specified direction.
    Move(Direction),
    /// Pull action in a specified direction.
    Pull(Direction),
}

impl Action for ReverseAction {
    fn direction(&self) -> Direction {
        match *self {
            ReverseAction::Move(direction) => direction,
            ReverseAction::Pull(direction) => direction,
        }
    }

    fn is_move(&self) -> bool {
        matches!(&self, ReverseAction::Move(_))
    }
}

impl ReverseAction {
    /// Checks if the action is a push action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::ReverseAction;
    /// use soukoban::direction::Direction;
    ///
    /// let action = ReverseAction::Pull(Direction::Up);
    /// assert!(action.is_pull());
    /// ```
    pub fn is_pull(&self) -> bool {
        matches!(&self, ReverseAction::Pull(_))
    }
}

impl TryFrom<char> for ForwardAction {
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
            Ok(ForwardAction::Push(direction))
        } else {
            Ok(ForwardAction::Move(direction))
        }
    }
}

impl From<ForwardAction> for char {
    fn from(action: ForwardAction) -> Self {
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

impl fmt::Display for ForwardAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<char>::into(*self))
    }
}
