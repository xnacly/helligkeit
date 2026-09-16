//! Display Data Channel abstraction, see https://en.wikipedia.org/wiki/Display_Data_Channel and https://milek7.pl/ddcbacklight/mccs.pdf

use std::{fmt::Display, io, path::Path};

use helligkeit_shared::Device;

pub struct Edid {
    name: String,
}

pub struct Ddc {
    dev: helligkeit_i2c::Dev,
    /// cached edid fetched at [Ddc::dev] creation time
    edid: Edid,
    /// cached max fetched at [Ddc::dev] creation time
    max: usize,
}

impl Ddc {
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        todo!()
    }
}

impl Display for Ddc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ddc")
    }
}

impl Device for Ddc {
    fn name(&self) -> &str {
        &self.edid.name
    }

    fn get(&self) -> std::io::Result<usize> {
        todo!()
    }

    fn set(&self, b: usize) -> std::io::Result<()> {
        todo!()
    }

    fn max(&self) -> io::Result<usize> {
        Ok(self.max)
    }
}

pub fn ddc() -> io::Result<impl Iterator<Item = Result<Ddc, io::Error>>> {
    Ok(std::iter::empty())
}
