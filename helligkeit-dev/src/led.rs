use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::PathBuf,
};

use helligkeit_shared::Device;

use crate::util;

pub const LED: &'static str = "/sys/class/leds";
const BRIGHTNESS: &'static str = "brightness";
const MAX_BRIGHTNESS: &'static str = "max_brightness";

#[derive(Debug)]
pub struct Led {
    pub name: String,
    pub path: PathBuf,
    pub max_brightness: usize,
}

impl Device for Led {
    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self) -> Result<usize, io::Error> {
        util::number_from_file(self.path.join(BRIGHTNESS))
    }

    fn max(&self) -> io::Result<usize> {
        Ok(self.max_brightness)
    }

    fn set(&self, b: usize) -> io::Result<()> {
        if b > self.max_brightness {
            if b > self.max_brightness {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "brightness exceeds max_brightness for the given device",
                ));
            }
        }

        let mut buf = Vec::with_capacity(16);
        writeln!(buf, "{b}")?;
        fs::write(self.path.join(BRIGHTNESS), buf)
    }
}

impl Display for Led {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        let brightness = self.get().unwrap_or_default();
        writeln!(
            f,
            "\tCurrent brightness: {} ({}%)",
            brightness,
            (brightness * 100) / self.max_brightness
        )?;
        writeln!(f, "\tMax brightness: {}", self.max_brightness)
    }
}

impl TryFrom<PathBuf> for Led {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let name = path
            .file_name()
            .ok_or_else(|| {
                Self::Error::new(io::ErrorKind::InvalidFilename, path.to_string_lossy())
            })?
            .to_string_lossy()
            .into_owned();

        let max_brightness = util::number_from_file(path.join(MAX_BRIGHTNESS))?;

        Ok(Led {
            name,
            path,
            max_brightness,
        })
    }
}
