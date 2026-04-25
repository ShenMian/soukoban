//! A direction.

use std::ops::Neg;

use crate::Vector2;

/// A direction.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    /// Upward direction (negative Y-axis).
    Up,
    /// Rightward direction (positive X-axis).
    Right,
    /// Downward direction (positive Y-axis).
    Down,
    /// Leftward direction (negative X-axis).
    Left,
}

impl Direction {
    /// Returns an iterator over all directions.
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Right, Self::Down, Self::Left].into_iter()
    }

    /// Rotates the direction 90° clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use soukoban::prelude::*;
    /// assert_eq!(Direction::Up.rotate_cw(), Direction::Right);
    ///
    /// // Rotate the direction 90° counter-clockwise.
    /// assert_eq!(-Direction::Right.rotate_cw(), Direction::Up);
    /// ```
    pub const fn rotate_cw(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// Rotates the direction 90° counter-clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use soukoban::prelude::*;
    /// assert_eq!(Direction::Up.rotate_ccw(), Direction::Left);
    /// ```
    pub fn rotate_ccw(self) -> Self {
        -self.rotate_cw()
    }

    /// Flips the direction horizontally.
    ///
    /// # Examples
    ///
    /// ```
    /// # use soukoban::prelude::*;
    /// assert_eq!(Direction::Left.flip_horizontal(), Direction::Right);
    /// assert_eq!(Direction::Up.flip_horizontal(), Direction::Up);
    /// ```
    pub const fn flip_horizontal(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up | Self::Down => self,
        }
    }

    /// Flips the direction vertically.
    ///
    /// # Examples
    ///
    /// ```
    /// # use soukoban::prelude::*;
    /// assert_eq!(Direction::Up.flip_vertical(), Direction::Down);
    /// assert_eq!(Direction::Left.flip_vertical(), Direction::Left);
    /// ```
    pub const fn flip_vertical(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left | Self::Right => self,
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.flip_horizontal().flip_vertical()
    }
}

impl From<Direction> for Vector2<i32> {
    fn from(direction: Direction) -> Self {
        use Direction as E;
        match direction {
            E::Up => -Self::y(),
            E::Right => Self::x(),
            E::Down => Self::y(),
            E::Left => -Self::x(),
        }
    }
}

impl TryFrom<Vector2<i32>> for Direction {
    type Error = ();

    fn try_from(vector: Vector2<i32>) -> Result<Self, Self::Error> {
        use Direction::*;
        match vector {
            v if v == -Vector2::<i32>::y() => Ok(Up),
            v if v == Vector2::<i32>::x() => Ok(Right),
            v if v == Vector2::<i32>::y() => Ok(Down),
            v if v == -Vector2::<i32>::x() => Ok(Left),
            _ => Err(()),
        }
    }
}

/// A directed position.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct DirectedPosition {
    /// The position.
    pub position: Vector2<i32>,
    /// The direction.
    pub direction: Direction,
}

impl DirectedPosition {
    /// Creates a new `DirectedPosition`.
    pub const fn new(position: Vector2<i32>, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}
