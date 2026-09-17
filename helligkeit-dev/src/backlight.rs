use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use helligkeit_shared::{Class, Device, IRRELEVANT};

use crate::util;

pub const BACKLIGHT: &'static str = "/sys/class/backlight";
/// last brightness requested by userspace. `actual_brightness` returns what the hardware applied.
/// Reading it wakes runtime suspended gpus (took 2.5s on my NVIDIA GeForce RTX 5080), so I pivoted
/// to brightness
const BRIGHTNESS: &'static str = "brightness";
const MAX_BRIGHTNESS: &'static str = "max_brightness";
/// one of `firmware`, `platform` or `raw`, see
/// https://www.kernel.org/doc/html/latest/gpu/backlight.html
const TYPE: &'static str = "type";

#[derive(Debug)]
pub struct Backlight {
    pub name: String,
    pub path: PathBuf,
    pub max_brightness: usize,
    /// backlight control interface, `firmware`, `platform` or `raw`
    pub kind: Option<String>,
    /// whether a display is attached to the panel this backlight controls,
    /// `None` if that could not be determined
    pub connected: Option<bool>,
}

/// Resolve whether the device behind a backlight is a connected display.
///
/// `<backlight>/device` either points at a drm connector directly or at the gpu (nvidia). I
/// consider a backlight connected if any connector it can be mapped to reports `connected`. Devices
/// without drm connectors (acpi_video) yield `None`.
fn connected(path: &Path) -> Option<bool> {
    let device = fs::canonicalize(path.join("device")).ok()?;

    // device is the connector itself
    if let Ok(status) = fs::read_to_string(device.join("status")) {
        return Some(status.trim() == "connected");
    }

    // device is the gpu, scan its connectors
    let cards = fs::read_dir(device.join("drm")).ok()?;
    let mut found = false;
    let mut any_connected = false;
    for card in cards.flatten() {
        let Ok(connectors) = fs::read_dir(card.path()) else {
            continue;
        };
        for connector in connectors.flatten() {
            if let Ok(status) = fs::read_to_string(connector.path().join("status")) {
                found = true;
                any_connected |= status.trim() == "connected";
            }
        }
    }

    found.then_some(any_connected)
}

impl Device for Backlight {
    fn name(&self) -> &str {
        &self.name
    }

    fn class(&self) -> Class {
        Class::Backlight
    }

    /// prefer firmware over platform over raw interfaces, see
    /// https://www.kernel.org/doc/html/latest/gpu/backlight.html. Backlights
    /// whose display is known to be disconnected are irrelevant
    fn rank(&self) -> u8 {
        let kind = match self.kind.as_deref() {
            Some("firmware") => 0,
            Some("platform") => 1,
            Some("raw") => 2,
            _ => 3,
        };
        if self.connected == Some(false) {
            IRRELEVANT + kind
        } else {
            kind
        }
    }

    fn get(&self) -> io::Result<usize> {
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

impl Display for Backlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        if let Some(kind) = &self.kind {
            writeln!(f, "\tType: {kind}")?;
        }
        if let Some(connected) = self.connected {
            writeln!(
                f,
                "\tDisplay: {}",
                if connected {
                    "connected"
                } else {
                    "disconnected"
                }
            )?;
        }
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

impl TryFrom<PathBuf> for Backlight {
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
        let kind = fs::read_to_string(path.join(TYPE))
            .ok()
            .map(|s| s.trim().to_owned());
        let connected = connected(&path);

        Ok(Backlight {
            name,
            path,
            max_brightness,
            kind,
            connected,
        })
    }
}
