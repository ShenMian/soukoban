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
    /// use soukoban::prelude::*;
    ///
    /// let action = Action::Move(Direction::Up);
    /// assert_eq!(action.direction(), Direction::Up);
    /// ```
    pub const fn direction(&self) -> Direction {
        match *self {
            Self::Move(direction) | Self::Push(direction) => direction,
        }
    }

    /// Checks if the action is a move action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::prelude::*;
    ///
    /// let action = Action::Move(Direction::Up);
    /// assert!(action.is_move());
    /// ```
    pub const fn is_move(&self) -> bool {
        matches!(&self, Self::Move(_))
    }

    /// Checks if the action is a shift action.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::prelude::*;
    ///
    /// let action = Action::Push(Direction::Up);
    /// assert!(action.is_shift());
    /// ```
    pub const fn is_shift(&self) -> bool {
        matches!(&self, Self::Push(_))
    }

    /// Rotates the action's direction 90° clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::prelude::*;
    ///
    /// let action = Action::Move(Direction::Up);
    /// assert_eq!(action.rotate_cw(), Action::Move(Direction::Right));
    /// ```
    pub fn rotate_cw(self) -> Self {
        self.map(Direction::rotate_cw)
    }

    /// Rotates the action's direction 90° counter-clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::prelude::*;
    ///
    /// let action = Action::Move(Direction::Up);
    /// assert_eq!(action.rotate_ccw(), Action::Move(Direction::Left));
    /// ```
    pub fn rotate_ccw(self) -> Self {
        self.map(Direction::rotate_ccw)
    }

    /// Flips the action's direction horizontally.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::prelude::*;
    ///
    /// let action = Action::Move(Direction::Left);
    /// assert_eq!(action.flip_horizontal(), Action::Move(Direction::Right));
    /// ```
    pub fn flip_horizontal(self) -> Self {
        self.map(Direction::flip_horizontal)
    }

    /// Flips the action's direction vertically.
    ///
    /// # Examples
    ///
    /// ```
    /// use soukoban::prelude::*;
    ///
    /// let action = Action::Move(Direction::Up);
    /// assert_eq!(action.flip_vertical(), Action::Move(Direction::Down));
    /// ```
    pub fn flip_vertical(self) -> Self {
        self.map(Direction::flip_vertical)
    }

    /// Applies a transformation function to the `Direction`.
    fn map(self, f: impl Fn(Direction) -> Direction) -> Self {
        match self {
            Self::Move(direction) => Self::Move(f(direction)),
            Self::Push(direction) => Self::Push(f(direction)),
        }
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
            Ok(Self::Push(direction))
        } else {
            Ok(Self::Move(direction))
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
        if action.is_shift() {
            char.to_ascii_uppercase()
        } else {
            char
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
