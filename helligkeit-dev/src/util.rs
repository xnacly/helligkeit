use std::{fs, io, path::Path, str::FromStr};

pub fn number_from_file<T: FromStr>(path: impl AsRef<Path>) -> io::Result<T> {
    let bytes = fs::read(path)?;

    str::from_utf8(&bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8 data in file"))?
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number in file"))
}

pub fn percent(value: usize, max: usize) -> usize {
    if max == 0 { 0 } else { (value * 100) / max }
}
