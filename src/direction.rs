//! A direction.

use std::ops::Neg;

use crate::point::Point;

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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

impl From<Direction> for Point {
    fn from(direction: Direction) -> Self {
        use Direction as E;
        match direction {
            E::Up => Self::UP,
            E::Right => Self::RIGHT,
            E::Down => Self::DOWN,
            E::Left => Self::LEFT,
        }
    }
}

impl TryFrom<Point> for Direction {
    type Error = ();

    fn try_from(position: Point) -> Result<Self, Self::Error> {
        use Direction::*;
        match position {
            v if v == Point::UP => Ok(Up),
            v if v == Point::RIGHT => Ok(Right),
            v if v == Point::DOWN => Ok(Down),
            v if v == Point::LEFT => Ok(Left),
            _ => Err(()),
        }
    }
}

/// A directed position.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct DirectedPosition {
    /// The position.
    pub position: Point,
    /// The direction.
    pub direction: Direction,
}

impl DirectedPosition {
    /// Creates a new `DirectedPosition`.
    #[must_use]
    pub const fn new(position: Point, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}
