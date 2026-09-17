use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::PathBuf,
};

use helligkeit_shared::{Class, Device, IRRELEVANT};

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

    fn class(&self) -> Class {
        Class::Led
    }

    /// leds are mostly keyboard indicators and network activity lights and
    /// therefore always irrelevant. Within them keyboard backlights come
    /// first, keyboard lock indicators driven by the kernel itself last
    fn rank(&self) -> u8 {
        const LOCK_INDICATORS: [&str; 5] = ["capslock", "numlock", "scrolllock", "kana", "compose"];

        let function = self.name.rsplit("::").next().unwrap_or(&self.name);
        if function == "kbd_backlight" {
            IRRELEVANT
        } else if LOCK_INDICATORS.contains(&function) {
            IRRELEVANT + 2
        } else {
            IRRELEVANT + 1
        }
    }

    fn get(&self) -> Result<usize, io::Error> {
        util::number_from_file(self.path.join(BRIGHTNESS))
    }

    fn max(&self) -> io::Result<usize> {
        Ok(self.max_brightness)
    }

    fn set(&self, b: usize) -> io::Result<()> {
        if b > self.max_brightness {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "brightness exceeds max_brightness for the given device",
            ));
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
            util::percent(brightness, self.max_brightness)
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
