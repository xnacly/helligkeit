use helligkeit_shared::Device;

const BACKLIGHT: &'static str = "/sys/class/backlight";
pub struct Backlight();

impl Device for Backlight {
    fn name(&self) -> &str {
        todo!()
    }

    fn get(&self) -> std::io::Result<u32> {
        todo!()
    }

    fn max(&self) -> std::io::Result<u32> {
        todo!()
    }

    fn set(&self, b: u32) -> std::io::Result<()> {
        todo!()
    }
}
