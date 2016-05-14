//! This crate provides a way to calculate how much data is being written into `std::io::Write`.
//! This is mostly useful if you want to know how much space is necessary to serialize something
//! without actually allocating that space or writing data anywhere.
//!
//! Example usage:
//!
//! ```
//! use std::io::Write;
//! use written_size::WrittenSize;
//!
//! let mut ws = WrittenSize::new();
//! ws.write(&[1, 2, 3]).unwrap();
//! ws.write(&[1, 2, 3]).unwrap();
//!
//! assert!(ws.size() == 6);
//! ```
//!
//! If you want to write data to some other `Write` instance and at the same time calculating number of bytes written you can use
//! this crate together with the [`broadcast`](https://crates.io/crates/broadcast) crate.
//!
//! If you want to calculate number of bytes read from some `Read` instance you can use this crate  together with the
//! [`tee`](https://crates.io/crates/tee) crate.
//!

use std::io;

/// Simple wrapper around `u64` which implements `std::io::Write`. It calculates how much data was written
/// discarding the data itself.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default, Debug)]
pub struct WrittenSize(pub u64);

impl WrittenSize {
    /// Create new instance with counter initialized to zero.
    #[inline]
    pub fn new() -> WrittenSize {
        WrittenSize(0)
    }

    /// Get computed size.
    #[inline]
    pub fn size(self) -> u64 {
        self.0
    }
}

impl io::Write for WrittenSize {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0 += buf.len() as u64;
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0 += buf.len() as u64;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::iter;
    use std::io::Write;
    use ::WrittenSize;

    #[test]
    fn test_100() {
        let max = 100;
        let data : Vec<u8> = iter::repeat(8).take(max).collect();

        for i in 0..100 {
            let mut ws = WrittenSize::new();
            assert!(ws.size() == 0);
            ws.write(&data[0..i]).unwrap();
            assert!(ws.size() == i as u64);
            ws.write_all(&[0, 1]).unwrap();
            assert!(ws.size() == (i + 2) as u64);
        }
    }
}
