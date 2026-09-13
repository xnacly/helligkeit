use std::io;

pub trait Device {
    fn name(&self) -> &str;
    fn get(&self) -> io::Result<u32>;
    fn max(&self) -> io::Result<u32>;
    fn set(&self, b: u32) -> io::Result<()>;
}
