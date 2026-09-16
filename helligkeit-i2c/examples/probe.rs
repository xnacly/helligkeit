use std::{fs, io::Write};

use helligkeit_i2c::Dev;

fn main() -> std::io::Result<()> {
    let mut edid = [0u8; 128];
    for dev in fs::read_dir("/dev")?
        .flatten()
        .filter(|f| {
            f.path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .starts_with("i2c-")
        })
        .filter_map(|f| Dev::start(f.path()).ok())
    {
        if dev.write_read(0x50, &[0x00], &mut edid).is_err() {
            // eprintln!("Failed to write_read({:?})", dev);
            continue;
        }

        std::io::stdout().write_all(&edid)?
    }

    Ok(())
}
