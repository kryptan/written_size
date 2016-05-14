use std::io;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default, Debug)]
pub struct WrittenSize(u64);

impl WrittenSize {
    pub fn new() -> WrittenSize {
        WrittenSize(0)
    }

    pub fn size(self) -> u64 {
        self.0
    }
}

impl io::Write for WrittenSize {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0 += buf.len() as u64;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
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
            ws.write(&[0]).unwrap();
            assert!(ws.size() == (i + 1) as u64);
        }
    }
}
