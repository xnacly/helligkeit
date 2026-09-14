use std::fmt::Display;

use helligkeit_shared::Device;

const BACKLIGHT: &'static str = "/sys/class/backlight";
pub struct Backlight();

impl Display for Backlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "backlight")
    }
}

impl Device for Backlight {
    fn name(&self) -> &str {
        todo!()
    }

    fn get(&self) -> std::io::Result<usize> {
        todo!()
    }

    fn max(&self) -> std::io::Result<usize> {
        todo!()
    }

    fn set(&self, b: usize) -> std::io::Result<()> {
        todo!()
    }
}
