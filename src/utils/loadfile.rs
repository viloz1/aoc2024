use std::{fs::File, io::BufRead, path::Path};
use std::io;

pub fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    lines.collect::<Result<Vec<_>, _>>()
}
