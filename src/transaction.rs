/// Wrapper around a AD9361 Transaction
use core::fmt;

pub struct Ad9361Transaction<'a>(pub &'a [u8]);

impl<'a> Ad9361Transaction<'a> {
    pub fn register(&self) -> u16 {
        self.0[1] as u16 + ((self.0[0] as u16 & 3) << 8)
    }
    pub fn is_write(&self) -> bool {
        self.0[0] & 0x80 > 0
    }
    pub fn value(&self) -> u8 {
        self.0[2]
    }
    pub fn length(&self) -> usize {
        ((self.0[0] >> 4) & 7) as usize + 1
    }
}

impl<'a> fmt::Debug for Ad9361Transaction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_write() {
            f.write_str("write ")?;
            f.write_fmt(format_args!("reg 0x{:03x} ", self.register()))?;
            f.write_fmt(format_args!("= 0x{:02x}", self.0[2]))
        } else {
            f.write_str("read  ")?;
            f.write_fmt(format_args!("reg 0x{:03x}", self.register()))
        }
    }
}
