use pest::Parser;
use pest_derive::Parser;
use std::fmt;

/// Single digital signal bit following the IEEE 1164 representation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Uninitialized,
    ForcingUnknown,
    Forcing0,
    Forcing1,
    HighImpedance,
    WeakUnknown,
    Weak0,
    Weak1,
    DontCare,
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Bit::Uninitialized => "U",
            Bit::ForcingUnknown => "X",
            Bit::Forcing0 => "0",
            Bit::Forcing1 => "1",
            Bit::HighImpedance => "Z",
            Bit::WeakUnknown => "W",
            Bit::Weak0 => "L",
            Bit::Weak1 => "H",
            Bit::DontCare => "-",
        };
        write!(f, "{}", s)
    }
}

pub struct Bits {
    bits: Vec<Bit>,
}

impl Bits {
    /// Creates a new, empty `Bits`.
    pub fn new() -> Self {
        Bits { bits: Vec::new() }
    }

    /// Adds a bit to the high-order end of the `Bits`.
    pub fn push(&mut self, bit: Bit) {
        self.bits.push(bit);
    }

    /// Returns the number of bits in the `Bits`.
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns true if the `Bits` contains no bits.
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Truncates the `Bits` to the specified length.
    pub fn truncate(&mut self, len: usize) {
        self.bits.truncate(len);
    }

    /// Zero-extends the `Bits` to the specified length.
    pub fn zext(&mut self, new_len: usize) {
        assert!(new_len > self.len());
        let additional_bits = new_len - self.len();
        self.bits
            .extend(std::iter::repeat(Bit::Forcing0).take(additional_bits));
    }

    /// Sign-extends the `Bits` to the specified length.
    pub fn sext(&mut self, new_len: usize) {
        assert!(new_len > self.len());
        if let Some(&last_bit) = self.bits.last() {
            assert!(last_bit == Bit::Forcing0 || last_bit == Bit::Forcing1);
            let additional_bits = new_len - self.len();
            self.bits
                .extend(std::iter::repeat(last_bit).take(additional_bits));
        }
    }
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // display bits so that the most significant bit is on the left
        for bit in self.bits.iter().rev() {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_display() {
        assert_eq!(Bit::Uninitialized.to_string(), "U");
        assert_eq!(Bit::ForcingUnknown.to_string(), "X");
        assert_eq!(Bit::Forcing0.to_string(), "0");
        assert_eq!(Bit::Forcing1.to_string(), "1");
        assert_eq!(Bit::HighImpedance.to_string(), "Z");
        assert_eq!(Bit::WeakUnknown.to_string(), "W");
        assert_eq!(Bit::Weak0.to_string(), "L");
        assert_eq!(Bit::Weak1.to_string(), "H");
        assert_eq!(Bit::DontCare.to_string(), "-");
    }

    #[test]
    fn test_bitvec_display() {
        let mut bv = Bits::new();
        bv.push(Bit::Forcing0);
        bv.push(Bit::Forcing1);
        bv.push(Bit::Forcing0);
        bv.push(Bit::Forcing1);
        assert_eq!(bv.to_string(), "1010");
    }

    #[test]
    fn test_bitvec_len() {
        let mut bv = Bits::new();
        assert_eq!(bv.len(), 0);
        bv.push(Bit::Forcing0);
        assert_eq!(bv.len(), 1);
        bv.push(Bit::Forcing1);
        assert_eq!(bv.len(), 2);
    }

    #[test]
    fn test_bitvec_is_empty() {
        let mut bv = Bits::new();
        assert!(bv.is_empty());
        bv.push(Bit::Forcing0);
        assert!(!bv.is_empty());
    }
}
