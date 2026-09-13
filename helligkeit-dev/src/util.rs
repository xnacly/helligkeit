use std::{fs, io, path::Path, str::FromStr};

pub fn number_from_file<T: FromStr>(path: impl AsRef<Path>) -> io::Result<T> {
    let bytes = fs::read(path)?;

    str::from_utf8(&bytes)
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?
        .trim()
        .parse()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))
}
