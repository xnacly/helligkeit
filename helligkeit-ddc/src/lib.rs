//! Display Data Channel abstraction, see https://en.wikipedia.org/wiki/Display_Data_Channel and https://milek7.pl/ddcbacklight/mccs.pdf

use std::io;

use helligkeit_shared::Device;

pub struct Ddc {}

impl Device for Ddc {
    fn name(&self) -> &str {
        todo!()
    }

    fn get(&self) -> std::io::Result<u32> {
        todo!()
    }

    fn set(&self, b: u32) -> std::io::Result<()> {
        todo!()
    }

    fn max(&self) -> io::Result<u32> {
        todo!()
    }
}

pub fn ddc() -> io::Result<impl Iterator<Item = Result<Ddc, io::Error>>> {
    Ok(std::iter::empty())
}
