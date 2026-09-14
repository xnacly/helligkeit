//! Display Data Channel abstraction, see https://en.wikipedia.org/wiki/Display_Data_Channel and https://milek7.pl/ddcbacklight/mccs.pdf

use std::{fmt::Display, io};

use helligkeit_shared::Device;

pub struct Ddc {}

impl Display for Ddc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ddc")
    }
}

impl Device for Ddc {
    fn name(&self) -> &str {
        todo!()
    }

    fn get(&self) -> std::io::Result<usize> {
        todo!()
    }

    fn set(&self, b: usize) -> std::io::Result<()> {
        todo!()
    }

    fn max(&self) -> io::Result<usize> {
        todo!()
    }
}

pub fn ddc() -> io::Result<impl Iterator<Item = Result<Ddc, io::Error>>> {
    Ok(std::iter::empty())
}
