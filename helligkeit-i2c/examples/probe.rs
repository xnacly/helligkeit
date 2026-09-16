use std::{fs, io::Write};

use helligkeit_i2c::Dev;

/// copied from [helligkeit_ddc::edid::HEADER] since ddc depends on i2c, otherwise this would be a
/// cyclic dependency
pub const HEADER: [u8; 8] = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];

/// executable via:
///
/// ```shell
/// cargo build -p helligkeit-i2c --example probe
/// sudo ./target/debug/examples/probe | hexdump -C
/// ```
///
/// results in:
///
/// ```text
/// 00000000  00 ff ff ff ff ff ff 00  06 b3 af 24 45 17 01 00  |...........$E...|
/// 00000010  0f 1e 01 03 80 35 1e 78  2a 51 b5 a4 54 4f a0 26  |.....5.x*Q..TO.&|
/// 00000020  0d 50 54 bf cf 00 81 40  81 80 95 00 71 4f 81 c0  |.PT....@....qO..|
/// 00000030  b3 00 01 01 01 01 02 3a  80 18 71 38 2d 40 58 2c  |.......:..q8-@X,|
/// 00000040  45 00 0f 28 21 00 00 1e  fc 7e 80 88 70 38 12 40  |E..(!....~..p8.@|
/// 00000050  18 20 35 00 0f 28 21 00  00 1e 00 00 00 fd 00 30  |. 5..(!........0|
/// 00000060  90 1e b4 22 00 0a 20 20  20 20 20 20 00 00 00 fc  |..."..      ....|
/// 00000070  00 41 53 55 53 20 56 50  32 34 39 0a 20 20 01 2b  |.ASUS VP249.  .+|
/// 00000080
/// ```
fn main() -> std::io::Result<()> {
    let mut edid = [0u8; 128];
    let mut buf = [0u8; 8];

    for dev in fs::read_dir("/dev")?
        .flatten() // only care about successful dir enum
        .filter(|f| {
            // check if /dev/ path starts i2c-
            f.path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .starts_with("i2c-")
        })
        // we only care if dev can be created
        .filter_map(|f| Dev::start(f.path()).ok())
    {
        // first fetch 8 bytes
        if dev.write_read(0x50, &[0x00], &mut buf).is_err() {
            continue;
        }

        // check if first 8 bytes match EDID header
        if buf != HEADER {
            continue;
        }

        // fetch the rest of EDID
        if dev.write_read(0x50, &[0x00], &mut edid).is_err() {
            continue;
        }

        std::io::stdout().write_all(&edid)?
    }

    Ok(())
}
