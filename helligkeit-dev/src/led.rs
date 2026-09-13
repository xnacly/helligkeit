use std::{fmt::Display, io, path::PathBuf};

use crate::util;

pub const LED: &'static str = "/sys/class/leds";
const BRIGHTNESS: &'static str = "brightness";
const MAX_BRIGHTNESS: &'static str = "max_brightness";

#[derive(Debug)]
pub struct Led {
    pub devicename: String,
    pub color: String,
    pub function: String,
    pub path: PathBuf,
    pub brightness: u32,
    pub max_brightness: u32,
}

impl Display for Led {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[led] {}", self.devicename,)?;
        writeln!(
            f,
            "\tCurrent brightness: {} ({}%)",
            self.brightness,
            (self.brightness * 100) / self.max_brightness
        )?;
        writeln!(f, "\tMax brightness: {}", self.max_brightness)
    }
}

impl TryFrom<PathBuf> for Led {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let name = path
            .file_name()
            .ok_or_else(|| Self::Error::from(io::ErrorKind::InvalidFilename))?
            .to_string_lossy();
        let mut name_split = name.split(':');

        let (devicename, color, function) = (
            name_split
                .next()
                .ok_or_else(|| Self::Error::from(io::ErrorKind::InvalidFilename))?
                .to_owned(),
            name_split
                .next()
                .ok_or_else(|| Self::Error::from(io::ErrorKind::InvalidFilename))?
                .to_owned(),
            name_split
                .last()
                .ok_or_else(|| Self::Error::from(io::ErrorKind::InvalidFilename))?
                .to_owned(),
        );

        let max_brightness = util::number_from_file(path.join(MAX_BRIGHTNESS))?;
        let brightness = util::number_from_file(path.join(BRIGHTNESS))?;

        Ok(Led {
            devicename,
            color,
            function,
            path,
            brightness,
            max_brightness,
        })
    }
}
