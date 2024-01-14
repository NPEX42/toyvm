pub mod types;

use std::{fs, path::PathBuf, io};

pub fn read_bytes(path: PathBuf) -> io::Result<Vec<u8>> {
    return fs::read(path);
}