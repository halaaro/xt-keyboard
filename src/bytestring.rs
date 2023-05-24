use core::fmt::Write;
use core::ops::Deref;

#[derive(Clone, Debug)]
pub struct ByteStringWriter<const N: usize> {
    buf: [u8; N],
    cursor: usize,
}

impl<const N: usize> ByteStringWriter<N> {
    pub fn new(buf: [u8; N]) -> Self {
        Self { buf, cursor: 0 }
    }
}

impl Default for ByteStringWriter<32> {
    fn default() -> Self {
        Self {
            buf: [b' '; 32],
            cursor: 0,
        }
    }
}

impl<const N: usize> Deref for ByteStringWriter<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        core::str::from_utf8(&self.buf).unwrap()
    }
}
impl<const N: usize> Write for ByteStringWriter<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            if self.cursor >= N {
                continue;
            } // silently fail!
            self.buf[self.cursor] = *b;
            self.cursor += 1;
        }
        Ok(())
    }
}
