use std::{fs, io};

mod util;

mod backlight;
pub use backlight::Backlight;

mod led;
pub use led::Led;

pub fn backlight() -> io::Result<impl Iterator<Item = Result<Backlight, io::Error>>> {
    let dir = fs::read_dir(backlight::BACKLIGHT)?;

    Ok(dir.map(|entry| {
        let entry = entry?;
        Backlight::try_from(entry.path())
    }))
}

pub fn leds() -> io::Result<impl Iterator<Item = Result<Led, io::Error>>> {
    let dir = fs::read_dir(led::LED)?;

    Ok(dir.map(|entry| {
        let entry = entry?;
        Led::try_from(entry.path())
    }))
}
