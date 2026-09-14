use std::{fmt::Display, io};

pub trait Device: Display {
    fn name(&self) -> &str;
    /// get brightness
    fn get(&self) -> io::Result<usize>;
    /// get max brightness
    fn max(&self) -> io::Result<usize>;
    /// set brightness
    fn set(&self, b: usize) -> io::Result<()>;
}
