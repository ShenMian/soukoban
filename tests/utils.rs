use std::{fs::File, io::BufReader, path::Path};

use soukoban::prelude::*;

/// Loads a level from a file using a 1-based level ID.
///
/// # Panics
///
/// Panics if the `id` is `0`, if the file at the given `path` cannot be opened,
/// or if the level cannot be read or parsed successfully.
pub fn load_level_from_file<P: AsRef<Path>>(path: P, id: usize) -> Level {
    debug_assert!(id >= 1);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    // Convert 1-based level ID to 0-based index.
    Level::load_nth_from_reader(reader, id - 1).unwrap()
}
