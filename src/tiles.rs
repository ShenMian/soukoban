//! Flags which can represent elements contained in map cells.

use bitflags::bitflags;

bitflags! {
    /// Flags which can represent elements contained in map cells.
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
