use std::{fs::File, io::BufReader, path::Path};

use soukoban::prelude::*;

pub fn load_level_from_file<P: AsRef<Path>>(path: P, id: usize) -> Level {
    debug_assert!(id >= 1);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    // Convert 1-based level ID to 0-based index.
    Level::load_nth_from_reader(reader, id - 1).unwrap()
}
