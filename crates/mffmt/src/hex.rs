use ::core::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexBytes<'a>(pub &'a [u8]);

impl<'a> Display for HexBytes<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut accum = 0u64;
        // remaining bytes after truncation.
        let rem_bytes = self.0.len() % 8;
        // byte len after truncated to a multiple of 8.
        let trunc_bytes = self.0.len() - rem_bytes;
        for i in (0..trunc_bytes).step_by(8) {
            for m in 0..8 {
                let shift = 7 - m;
                let byte_index = i + m;
                let byte = self.0[byte_index];
                accum |= (byte as u64) << (shift * 8);
            }
            write!(f, "{accum:016x}")?;
            accum = 0;
        }
        for i in trunc_bytes..self.0.len() {
            let byte = self.0[i];
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

#[inline(always)]
#[must_use]
pub const fn hex<'a>(bytes: &'a [u8]) -> HexBytes<'a> {
    HexBytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn take_fn<'a, R, F: FnOnce(&'a [u8]) -> R>(bytes: &'a [u8], f: F) -> R {
        f(bytes)
    }
    
    #[test]
    fn hex_bytes_test() {
        let bytes = [0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA];
        let hex_string_take_fn = take_fn(&bytes, HexBytes);
        let hex_string = format!("{}:{}", hex(&bytes), hex_string_take_fn);
        assert_eq!(hex_string, "a0a1a2a3a4a5a6a7a8a9aa:a0a1a2a3a4a5a6a7a8a9aa");
    }
}