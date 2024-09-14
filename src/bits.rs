use pest::Parser;
use pest_derive::Parser;
use std::fmt;

/// Single digital signal bit following the IEEE 1164 representation.
enum Bit {
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

struct VerilogHexNumberAst {
    size: Option<usize>,
    signed: Option<bool>, // from hex_base
    hex_value: Vec<Bit>,
}

#[derive(Parser)]
#[grammar = "parser/verilog_literal.pest"]
struct VerilogLiteralParser;

impl Bits {

    fn from_verilog_hex(input: &str) -> Result<Bits, pest::error::Error<Rule>> {
        let pairs = VerilogLiteralParser::parse(Rule::hex_number, input)?.next().unwrap().into_inner();
        println!("pairs: {:#?}", pairs);

        let mut bits = Bits::new();
        let mut size = None;
        let mut signed = false;

        for pair in pairs {
            match pair.as_rule() {
                Rule::size => {
                    size = Some(pair.as_str().parse::<usize>().unwrap());
                    println!("size: {:?}", size);
                },
                Rule::hex_base => {
                    if pair.as_str().to_lowercase().contains('s') {
                        signed = true;
                    }
                    let hex_base_str = pair.as_str();
                    println!("hex_base: {}", hex_base_str);
                },
                Rule::hex_value => {
                    // iterate hex digits from right to left

                    for digit in pair.into_inner().rev() {
                        match digit.as_rule() {
                            Rule::hex_digit => {
                                let hex_char = digit.as_str().chars().next().unwrap();
                                println!("hex_char: {}", hex_char);
                                let char_bits = match hex_char {
                                    // least-significant comes first in the bit vector
                                    '0'         => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing0, Bit::Forcing0],
                                    '1'         => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing0, Bit::Forcing0],
                                    '2'         => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing0, Bit::Forcing0],
                                    '3'         => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing0, Bit::Forcing0],
                                    '4'         => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing1, Bit::Forcing0],
                                    '5'         => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing1, Bit::Forcing0],
                                    '6'         => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing1, Bit::Forcing0],
                                    '7'         => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing1, Bit::Forcing0],
                                    '8'         => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing0, Bit::Forcing1],
                                    '9'         => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing0, Bit::Forcing1],
                                    'A' | 'a'   => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing0, Bit::Forcing1],
                                    'B' | 'b'   => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing0, Bit::Forcing1],
                                    'C' | 'c'   => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing1, Bit::Forcing1],
                                    'D' | 'd'   => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing1, Bit::Forcing1],
                                    'E' | 'e'   => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing1, Bit::Forcing1],
                                    'F' | 'f'   => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing1, Bit::Forcing1],
                                    '_' => continue,
                                    _ => unreachable!(),
                                };
                                for bit in char_bits {
                                    bits.push(bit);
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        // TODO: infer width
        // - when size is not specified, infer from hex_value
        // - when size is specified and hex_value is wider, truncate
        // - when size is specified and hex_value is narrower, fill high-order bits with 0
        
        // TODO: figure out how the sign affects high-order bits
        // TODO: do something with `signed` variable

        return Ok(bits);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_verilog_hex() {
        let bits = Bits::from_verilog_hex("1'h0").unwrap();
        assert_eq!(bits.to_string(), "0");
        
        let bits = Bits::from_verilog_hex("'h1").unwrap();
        assert_eq!(bits.to_string(), "1");
        
        let bits = Bits::from_verilog_hex("'h6").unwrap();
        assert_eq!(bits.to_string(), "110");

        let bits = Bits::from_verilog_hex("5'h6").unwrap();
        assert_eq!(bits.to_string(), "00110");

        let bits = Bits::from_verilog_hex("13'hF03").unwrap();
        assert_eq!(bits.to_string(), "11100000011");

        let bits = Bits::from_verilog_hex("32'h1234_5678").unwrap();
        assert_eq!(bits.to_string(), "10000111011001010100001100100001");
        
        let bits = Bits::from_verilog_hex("5'sH8").unwrap();
        assert_eq!(bits.to_string(), "11000");
    }

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
