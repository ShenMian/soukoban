//! A direction.

use std::ops::Neg;

use nalgebra::Vector2;

/// A direction.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    /// Up.
    Up,
    /// Down.
    Down,
    /// Left.
    Left,
    /// Right.
    Right,
}

impl Direction {
    /// Returns an iterator over all directions.
    pub fn iter() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [Up, Down, Left, Right].iter().copied()
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

impl From<Direction> for Vector2<i32> {
    fn from(direction: Direction) -> Self {
        use Direction as E;
        match direction {
            E::Up => -Vector2::y(),
            E::Down => Vector2::y(),
            E::Left => -Vector2::x(),
            E::Right => Vector2::x(),
        }
    }
}

impl TryFrom<Vector2<i32>> for Direction {
    type Error = ();

    fn try_from(vector: Vector2<i32>) -> Result<Self, Self::Error> {
        use Direction::*;
        match vector {
            v if v == -Vector2::<i32>::y() => Ok(Up),
            v if v == Vector2::<i32>::y() => Ok(Down),
            v if v == -Vector2::<i32>::x() => Ok(Left),
            v if v == Vector2::<i32>::x() => Ok(Right),
            _ => Err(()),
        }
    }
}
