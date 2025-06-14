//! A direction.

use std::ops::Neg;

use crate::point::Point;

/// A direction.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    /// Upward direction (negative Y-axis).
    Up,
    /// Downward direction (positive Y-axis).
    Down,
    /// Leftward direction (negative X-axis).
    Left,
    /// Rightward direction (positive X-axis).
    Right,
}

impl Direction {
    /// Returns an iterator over all directions.
    pub fn iter() -> std::array::IntoIter<Direction, 4> {
        [Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
    }

    /// Rotate the direction 90° clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use soukoban::direction::Direction;
    /// assert_eq!(Direction::Up.rotate(), Direction::Right);
    ///
    /// // Rotate the direction 90° counter clockwis.
    /// assert_eq!(-Direction::Right.rotate(), Direction::Up);
    /// ```
    pub fn rotate(self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    /// Flip the direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use soukoban::direction::Direction;
    /// assert_eq!(Direction::Left.flip(), Direction::Right);
    /// assert_eq!(Direction::Up.flip(), Direction::Down);
    /// ```
    pub fn flip(self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl From<Direction> for Point {
    fn from(direction: Direction) -> Self {
        use Direction as E;
        match direction {
            E::Up => Point::new(0, -1),
            E::Down => Point::new(0, 1),
            E::Left => Point::new(-1, 0),
            E::Right => Point::new(1, 0),
        }
    }
}

impl TryFrom<Point> for Direction {
    type Error = ();

    fn try_from(vector: Point) -> Result<Self, Self::Error> {
        use Direction::*;
        match vector {
            v if v == Point::new(0, -1) => Ok(Up),
            v if v == Point::new(0, 1) => Ok(Down),
            v if v == Point::new(-1, 0) => Ok(Left),
            v if v == Point::new(1, 0) => Ok(Right),
            _ => Err(()),
        }
    }
}
