#![allow(dead_code)]

use std::{fs, path::Path};

use soukoban::Level;

pub fn load_level_from_file<P: AsRef<Path>>(path: P, id: usize) -> Level {
    debug_assert!(id >= 1);
    Level::load_nth_from_string(&fs::read_to_string(path).unwrap(), id).unwrap()
}
