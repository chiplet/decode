use std::fmt;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HexDigit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15
}

impl fmt::Display for HexDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HexDigit::Zero => "0",
            HexDigit::One => "1",
            HexDigit::Two => "2",
            HexDigit::Three => "3",
            HexDigit::Four => "4",
            HexDigit::Five => "5",
            HexDigit::Six => "6",
            HexDigit::Seven => "7",
            HexDigit::Eight => "8",
            HexDigit::Nine => "9",
            HexDigit::A => "A",
            HexDigit::B => "B",
            HexDigit::C => "C",
            HexDigit::D => "D",
            HexDigit::E => "E",
            HexDigit::F => "F",
        };
        write!(f, "{}", s)
    }
}

pub mod verilog {
    use std::fmt;
    use super::HexDigit;

    #[derive(Debug)]
    pub struct VerilogHexNumberAst {
        pub size: Option<usize>,
        pub signed: bool,
        pub hex_digits: Vec<HexDigit>,
    }

    impl fmt::Display for VerilogHexNumberAst {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let size_str = if let Some(size) = self.size {
                size.to_string()
            } else {
                "".to_string()
            };
            let signed_str = if self.signed { "s" } else { "" };
            let hex_str: String = self.hex_digits.iter().rev().map(|d| d.to_string()).collect();
            write!(f, "{}'h{}{}", size_str, signed_str, hex_str)
        }
    }
    
    impl VerilogHexNumberAst {
        pub fn new() -> Self {
            Self {
                size: None,
                signed: false,
                hex_digits: Vec::new(),
            }
        }

        /// Number of bits required to represent the hexadecimal number. If the size was specified
        /// in verilog syntax it is stored in `size` and that value is returned. If the size was
        /// not specified, it is inferred from `hex_digits` such as the minimum number of bits required
        /// to represent the binary number. High-order zeros are ignored when inferring the number of bits.
        pub fn size(&self) -> usize {
            self.size.unwrap_or_else(|| {
                // infer minimum number of bits required to represent given hex digits
                assert!(self.hex_digits.len() > 0);

                // edge case, one-bit wide zero
                if self.hex_digits.len() == 1 && self.hex_digits[0] == HexDigit::Zero {
                    1
                } else {
                    // count number of non-zero low-order digits
                    let num_nonzero_lo_digits = self.hex_digits.len() - self.hex_digits.iter().rev().position(|x| *x != HexDigit::Zero).unwrap();
                    let nonzero_lo_digits = &self.hex_digits[0..num_nonzero_lo_digits];

                    let (last_digit, lo_digits) = nonzero_lo_digits.split_last().unwrap();
                    
                    let num_last_digit_bits = match last_digit {
                        HexDigit::Zero => unreachable!(),
                        HexDigit::One => 1,
                        HexDigit::Two => 2,
                        HexDigit::Three => 2,
                        HexDigit::Four => 3,
                        HexDigit::Five => 3,
                        HexDigit::Six => 3,
                        HexDigit::Seven => 3,
                        HexDigit::Eight => 4,
                        HexDigit::Nine => 4,
                        HexDigit::A => 4,
                        HexDigit::B => 4,
                        HexDigit::C => 4,
                        HexDigit::D => 4,
                        HexDigit::E => 4,
                        HexDigit::F => 4,
                    };

                    let num_lo_bits = 4 * lo_digits.len();
                    
                    num_lo_bits + num_last_digit_bits
                }
            })
        }
    }
}
