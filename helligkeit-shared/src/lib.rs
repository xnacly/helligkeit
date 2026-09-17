use std::{fmt::Display, io};

/// Device category, ordered by how likely a user is to mean it when they
/// talk about "brightness": the internal panel first, external monitors
/// second, leds last
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    Backlight,
    Ddc,
    Led,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Backlight => write!(f, "backlight"),
            Class::Ddc => write!(f, "ddc"),
            Class::Led => write!(f, "led"),
        }
    }
}

/// Devices with a [Device::rank] at or above this are considered irrelevant:
/// they still work when addressed explicitly but are hidden from listings and
/// never picked as default, e.g. keyboard lock leds or the backlight of a gpu
/// without a connected display
pub const IRRELEVANT: u8 = 128;

pub trait Device: Display {
    fn name(&self) -> &str;
    fn class(&self) -> Class;
    /// tie-break within a class, lower is preferred; devices are ordered by
    /// ([Device::class], [Device::rank], [Device::name]). Ranks at or above
    /// [IRRELEVANT] mark the device as not interesting by default
    fn rank(&self) -> u8 {
        0
    }
    /// get brightness
    fn get(&self) -> io::Result<usize>;
    /// get max brightness
    fn max(&self) -> io::Result<usize>;
    /// set brightness
    fn set(&self, b: usize) -> io::Result<()>;
}

/// ordering key for a device, see [Device::rank]
pub fn sort_key(d: &dyn Device) -> (Class, u8, String) {
    (d.class(), d.rank(), d.name().to_owned())
}
