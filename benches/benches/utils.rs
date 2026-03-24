use soukoban::Level;
use std::{fs::File, io::BufReader, path::Path};

pub fn load_level_from_file<P: AsRef<Path>>(path: P, id: usize) -> Level {
    debug_assert!(id >= 1);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    Level::load_nth_from_reader(reader, id).unwrap()
}
