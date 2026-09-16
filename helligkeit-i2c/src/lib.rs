//! userland inter-integrated circuit abstraction, see https://en.wikipedia.org/wiki/I2C
//!
//! headers and definitions taken from:
//! - https://github.com/torvalds/linux/blob/master/include/uapi/linux/i2c-dev.h
//! - https://github.com/torvalds/linux/blob/704340f1cd0dcef829eb62f5b48ae95a2ce17bdf/include/uapi/linux/i2c.h
//!
//! - general i2c device interface guide at: https://www.kernel.org/doc/html/v5.4/i2c/dev-interface.html

use std::{
    fs::{File, OpenOptions},
    io,
    os::fd::AsRawFd,
    path::Path,
};

use core::ffi::{c_int, c_ulong};

/// %I2C_M_RD: read data (from slave to master). Guaranteed to be 0x0001! If
/// not set, the transaction is interpreted as write.
///
/// Optional:
/// %I2C_M_DMA_SAFE: the buffer of this message is DMA safe. Makes only sense
///   in kernelspace, because userspace buffers are copied anyway
///
/// Only if I2C_FUNC_10BIT_ADDR is set:
/// %I2C_M_TEN: this is a 10 bit chip address
///
/// Only if I2C_FUNC_SMBUS_READ_BLOCK_DATA is set:
/// %I2C_M_RECV_LEN: message length will be first received byte
///
/// Only if I2C_FUNC_NOSTART is set:
/// %I2C_M_NOSTART: skip repeated start sequence
///
/// Only if I2C_FUNC_PROTOCOL_MANGLING is set:
/// %I2C_M_NO_RD_ACK: in a read message, master ACK/NACK bit is skipped
/// %I2C_M_IGNORE_NAK: treat NACK from client as ACK
/// %I2C_M_REV_DIR_ADDR: toggles the Rd/Wr bit
/// %I2C_M_STOP: force a STOP condition after the message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct I2cFlags(u16);

impl I2cFlags {
    pub const READ: Self = Self(0x0001);
    pub const TEN_BIT: Self = Self(0x0010);
    pub const DMA_SAFE: Self = Self(0x0200);
    pub const RECV_LEN: Self = Self(0x0400);
    pub const NO_RD_ACK: Self = Self(0x0800);
    pub const IGNORE_NAK: Self = Self(0x1000);
    pub const REV_DIR_ADDR: Self = Self(0x2000);
    pub const NOSTART: Self = Self(0x4000);
    pub const STOP: Self = Self(0x8000);

    pub const fn bits(self) -> u16 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// /dev/i2c-X ioctl commands.The ioctl's parameter is always an
/// unsigned long, except for:
///  - I2C_FUNCS, takes pointer to an unsigned long
///  - I2C_RDWR, takes pointer to struct i2c_rdwr_ioctl_data
///  - I2C_SMBUS, takes pointer to struct i2c_smbus_ioctl_data
pub enum I2cIoctl {
    Retries = 0x0701,
    Timeout = 0x0702,
    Slave = 0x0703,
    TenBit = 0x0704,
    Funcs = 0x0705,
    SlaveForce = 0x0706,
    RdWr = 0x0707,
    Pec = 0x0708,
    Smbus = 0x0720,
}

unsafe extern "C" {
    /// see man 2 ioctl
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[repr(C)]
/// An I2C transaction segment beginning with START
///
/// An i2c_msg is the low level representation of one segment of an I2C
/// transaction.  It is visible to drivers in the @i2c_transfer() procedure,
/// to userspace from i2c-dev, and to I2C adapter drivers through the
/// @i2c_adapter.@master_xfer() method.
///
/// Except when I2C "protocol mangling" is used, all I2C adapters implement
/// the standard rules for I2C transactions.  Each transaction begins with a
/// START.  That is followed by the slave address, and a bit encoding read
/// versus write.  Then follow all the data bytes, possibly including a byte
/// with SMBus PEC.  The transfer terminates with a NAK, or when all those
/// bytes have been transferred and ACKed.  If this is the last message in a
/// group, it is followed by a STOP.  Otherwise it is followed by the next
/// @i2c_msg transaction segment, beginning with a (repeated) START.
///
/// Alternatively, when the adapter supports %I2C_FUNC_PROTOCOL_MANGLING then
/// passing certain @flags may have changed those standard protocol behaviors.
/// Those flags are only for use with broken/nonconforming slaves, and with
/// adapters which are known to support the specific mangling options they need.
///
/// Taken verbatim from https://github.com/torvalds/linux/blob/704340f1cd0dcef829eb62f5b48ae95a2ce17bdf/include/uapi/linux/i2c.h#L74 (i2c.h@i2c_msg)
struct I2cMsg {
    /// Slave address, either 7 or 10 bits. When this is a 10 bit address,
    /// %I2C_M_TEN must be set in flags and the adapter must support
    /// %I2C_FUNC_10BIT_ADDR.
    addr: u16,
    /// Supported by all adapters, see [I2cFlags]
    flags: u16,
    /// Number of data bytes in [I2cMsg::buf] being read from or written to the I2C
    /// slave address. For read transactions where %I2C_M_RECV_LEN is set, the
    /// caller guarantees that this buffer can hold up to %I2C_SMBUS_BLOCK_MAX
    /// bytes in addition to the initial length byte sent by the slave (plus,
    /// if used, the SMBus PEC); and this value will be incremented by the number
    /// of block data bytes received.
    len: u16,
    /// The buffer into which data is read, or from which it's written.
    buf: *mut u8,
}

#[repr(C)]
/// This is the structure as used in the I2C_RDWR ioctl call
///
/// Taken verbatim from https://github.com/torvalds/linux/blob/704340f1cd0dcef829eb62f5b48ae95a2ce17bdf/include/uapi/linux/i2c-dev.h#L50C8-L50C27 (i2c-dev.h@i2c_rdwr_ioctl_data)
struct I2cRdwrIoctlData {
    msgs: *mut I2cMsg,
    nmsgs: u32,
}

/// i2c Device
#[derive(Debug)]
pub struct Dev {
    fd: File,
}

impl Dev {
    pub fn start(path: impl AsRef<Path>) -> io::Result<Self> {
        let fd = OpenOptions::new().read(true).write(true).open(path)?;
        Ok(Self { fd })
    }

    pub fn write(&self, addr: u16, data: &[u8]) -> io::Result<()> {
        let mut msg = I2cMsg {
            addr,
            // 0 is WRITE
            flags: 0,
            len: u16::try_from(data.len())
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "data.len() > u16::MAX"))?,
            buf: data.as_ptr() as *mut u8,
        };

        let mut transfer = I2cRdwrIoctlData {
            msgs: &mut msg,
            nmsgs: 1,
        };

        let rc = unsafe {
            ioctl(
                self.fd.as_raw_fd(),
                I2cIoctl::RdWr as c_ulong,
                &mut transfer,
            )
        };

        if rc < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn read(&self, addr: u16, data: &mut [u8]) -> io::Result<()> {
        let mut msg = I2cMsg {
            addr: addr,
            flags: I2cFlags::READ.bits(),
            len: u16::try_from(data.len())
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "data.len() > u16::MAX"))?,
            buf: data.as_mut_ptr(),
        };

        let mut transfer_container = I2cRdwrIoctlData {
            msgs: &mut msg,
            nmsgs: 1,
        };

        let rc = unsafe {
            ioctl(
                self.fd.as_raw_fd(),
                I2cIoctl::RdWr as c_ulong,
                &mut transfer_container,
            )
        };

        if rc < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn write_read(&self, addr: u16, w: &[u8], r: &mut [u8]) -> io::Result<()> {
        // START
        //   address + WRITE
        //   write bytes
        // REPEATED START
        //   address + READ
        //   read bytes
        // STOP
        let mut msgs = [
            I2cMsg {
                addr: addr,
                flags: 0,
                len: u16::try_from(w.len()).map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "w.len() > u16::MAX")
                })?,
                buf: w.as_ptr() as *mut u8,
            },
            I2cMsg {
                addr: addr,
                flags: I2cFlags::READ.bits(),
                len: u16::try_from(r.len()).map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "r.len() > u16::MAX")
                })?,
                buf: r.as_mut_ptr(),
            },
        ];

        let mut transfer_container = I2cRdwrIoctlData {
            msgs: msgs.as_mut_ptr(),
            nmsgs: msgs.len() as u32,
        };

        let rc = unsafe {
            ioctl(
                self.fd.as_raw_fd(),
                I2cIoctl::RdWr as c_ulong,
                &mut transfer_container,
            )
        };

        if rc < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}
