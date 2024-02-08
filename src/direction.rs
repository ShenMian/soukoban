//! A direction.

use std::ops::Neg;

use nalgebra::Vector2;

/// A direction.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl From<Direction> for Vector2<i32> {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => -Vector2::y(),
            Down => Vector2::y(),
            Left => -Vector2::x(),
            Right => Vector2::x(),
        }
    }
}

impl TryFrom<Vector2<i32>> for Direction {
    type Error = ();

    fn try_from(vector: Vector2<i32>) -> Result<Self, Self::Error> {
        use Direction::*;
        if vector == -Vector2::<i32>::y() {
            Ok(Up)
        } else if vector == Vector2::<i32>::y() {
            Ok(Down)
        } else if vector == -Vector2::<i32>::x() {
            Ok(Left)
        } else if vector == Vector2::<i32>::x() {
            Ok(Right)
        } else {
            Err(())
        }
    }
}
