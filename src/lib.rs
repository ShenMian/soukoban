#![doc = include_str!("../README.md")]
#![allow(clippy::op_ref)]
#![deny(missing_docs, clippy::missing_panics_doc)]

pub mod action;
pub mod actions;
pub mod deadlock;
pub mod direction;
pub mod error;
pub mod level;
pub mod map;
pub mod path_finding;
pub mod point;
pub mod solver;
pub mod tiles;

mod bcc_graph;
mod run_length;

use action::*;
use actions::*;
use error::*;
use level::*;
use map::*;
use tiles::*;

pub use rustc_hash::{FxHashMap, FxHashSet};

/// Convenience re-export of common structs and functions.
pub mod prelude {
    pub use crate::action::*;
    pub use crate::actions::*;
    pub use crate::direction::*;
    pub use crate::error::*;
    pub use crate::level::*;
    pub use crate::map::*;
    pub use crate::point::*;
    pub use crate::tiles::*;
    pub use crate::{FxHashMap, FxHashSet};
}
