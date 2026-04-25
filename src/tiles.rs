//! Flags which can represent elements contained in map cells.

use std::fmt;

use bitflags::bitflags;

bitflags! {
    /// Flags which can represent elements contained in map cells.
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub struct Tiles: u8 {
        /// Floor.
        const Floor = 1 << 0;
        /// Wall.
        const Wall = 1 << 1;
        /// Box.
        const Box = 1 << 2;
        /// Goal.
        const Goal = 1 << 3;
        /// Player.
        const Player = 1 << 4;
    }
}

impl fmt::Display for Tiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tiles = *self;
        if tiles == Self::Floor {
            write!(f, "_")?;
            return Ok(());
        }
        tiles.remove(Self::Floor);
        if tiles == Self::Box | Self::Goal {
            write!(f, "*")?;
        } else if tiles == Self::Player | Self::Goal {
            write!(f, "+")?;
        } else if tiles == Self::Box {
            write!(f, "$")?;
        } else if tiles == Self::Goal {
            write!(f, ".")?;
        } else if tiles == Self::Player {
            write!(f, "@")?;
        } else if tiles == Self::Wall {
            write!(f, "#")?;
        } else if tiles.is_empty() {
            write!(f, "-")?;
        } else {
            write!(f, "?")?;
        }
        Ok(())
    }
}
