//! A position in 2D space.

use std::ops;

/// A position in 2D space.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    /// The x coordinate.
    pub x: i32,
    /// The y coordinate.
    pub y: i32,
}

impl Point {
    /// Creates a new `Point`.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns a new `Point` with the absolute values of its coordinates.
    #[must_use]
    pub fn abs(self) -> Self {
        self.map(i32::abs)
    }

    /// Returns the sum of the `x` and `y` coordinates.
    #[must_use]
    pub const fn sum(self) -> i32 {
        self.x + self.y
    }

    /// Returns a new `Point` with the `x` and `y` coordinates swapped.
    #[must_use]
    pub const fn yx(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    /// Returns a new `Point` containing the result of `f` applied to each
    /// of its entries.
    #[must_use]
    pub fn map<F>(&self, mut f: F) -> Self
    where
        F: FnMut(i32) -> i32,
    {
        Self {
            x: f(self.x),
            y: f(self.y),
        }
    }

    /// Returns a new `Point` containing the result of `f` applied to each
    /// entries of `self` and `rhs`.
    #[must_use]
    pub fn zip_map<F>(&self, rhs: &Point, mut f: F) -> Self
    where
        F: FnMut(i32, i32) -> i32,
    {
        Self {
            x: f(self.x, rhs.x),
            y: f(self.y, rhs.y),
        }
    }

    /// A `Point` at the origin `(0, 0)`.
    pub const ZERO: Self = Self { x: 0, y: 0 };
    /// Unit vector pointing up (negative y).
    pub const UP: Self = Self { x: 0, y: -1 };
    /// Unit vector pointing right.
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    /// Unit vector pointing left.
    pub const LEFT: Self = Self { x: -1, y: 0 };
    /// Unit vector pointing down (positive y).
    pub const DOWN: Self = Self { x: 0, y: 1 };
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        *self += *rhs;
    }
}

impl ops::SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        *self -= *rhs;
    }
}

impl ops::Add<&Point> for Point {
    type Output = Self;

    fn add(self, rhs: &Point) -> Self::Output {
        self + *rhs
    }
}

impl ops::Sub<&Point> for Point {
    type Output = Self;

    fn sub(self, rhs: &Point) -> Self::Output {
        self - *rhs
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        *self + *rhs
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        *self - *rhs
    }
}
