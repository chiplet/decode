use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::bits::{Bit, Bits};
use crate::frontend::ast::HexDigit;
use crate::frontend::ast::verilog::VerilogHexNumberAst;

#[derive(Parser)]
#[grammar = "parser/verilog_number.pest"]
struct VerilogNumberParser;

fn parse_hex_number(pair: Pair<Rule>) -> VerilogHexNumberAst {
    match pair.as_rule() {
        Rule::hex_number => (),
        _ => panic!("`pair` should be a successfully parsed `hex_number` rule in verilog syntax."),
    }
    let mut size = None;
    let mut signed = false;
    let mut hex_digits = Vec::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::size => {
                size = Some(pair.as_str().parse::<usize>().unwrap());
                log::debug!("size: {:?}", size);
            },
            Rule::hex_base => {
                if pair.as_str().to_lowercase().contains('s') {
                    signed = true;
                }
                let hex_base_str = pair.as_str();
                log::debug!("hex_base: {}", hex_base_str);
            },
            Rule::hex_value => {
                // hex digits are stored in little-endian order so iterate hex digits from right to left
                for digit in pair.into_inner().rev() {
                    match digit.as_rule() {
                        Rule::hex_digit => {
                            let hex_char = digit.as_str().chars().next().unwrap();
                            log::debug!("hex_char: {}", hex_char);
                            let hex_digit = match hex_char {
                                '0'         => HexDigit::Zero,
                                '1'         => HexDigit::One,
                                '2'         => HexDigit::Two,
                                '3'         => HexDigit::Three,
                                '4'         => HexDigit::Four,
                                '5'         => HexDigit::Five,
                                '6'         => HexDigit::Six,
                                '7'         => HexDigit::Seven,
                                '8'         => HexDigit::Eight,
                                '9'         => HexDigit::Nine,
                                'A' | 'a'   => HexDigit::A,
                                'B' | 'b'   => HexDigit::B,
                                'C' | 'c'   => HexDigit::C,
                                'D' | 'd'   => HexDigit::D,
                                'E' | 'e'   => HexDigit::E,
                                'F' | 'f'   => HexDigit::F,
                                '_' => continue,
                                _ => unreachable!(),
                            };
                            hex_digits.push(hex_digit);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    VerilogHexNumberAst {
        size,
        signed,
        hex_digits,
    }
}

fn verilog_number_to_bits(ast: VerilogHexNumberAst) -> Bits {
    let mut bits = Bits::new();
    for digit in &ast.hex_digits {
        let char_bits = match digit {
            // least-significant comes first in the bit vector
            HexDigit::Zero => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing0, Bit::Forcing0],
            HexDigit::One => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing0, Bit::Forcing0],
            HexDigit::Two => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing0, Bit::Forcing0],
            HexDigit::Three => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing0, Bit::Forcing0],
            HexDigit::Four => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing1, Bit::Forcing0],
            HexDigit::Five => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing1, Bit::Forcing0],
            HexDigit::Six => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing1, Bit::Forcing0],
            HexDigit::Seven => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing1, Bit::Forcing0],
            HexDigit::Eight => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing0, Bit::Forcing1],
            HexDigit::Nine => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing0, Bit::Forcing1],
            HexDigit::A => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing0, Bit::Forcing1],
            HexDigit::B => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing0, Bit::Forcing1],
            HexDigit::C => vec![Bit::Forcing0, Bit::Forcing0, Bit::Forcing1, Bit::Forcing1],
            HexDigit::D => vec![Bit::Forcing1, Bit::Forcing0, Bit::Forcing1, Bit::Forcing1],
            HexDigit::E => vec![Bit::Forcing0, Bit::Forcing1, Bit::Forcing1, Bit::Forcing1],
            HexDigit::F => vec![Bit::Forcing1, Bit::Forcing1, Bit::Forcing1, Bit::Forcing1],
        };
        for bit in char_bits {
            bits.push(bit);
        }
    }

    // infer number of bits if it's not explicitly specified
    let num_bits = ast.size();

    if bits.len() > num_bits {
        // when size is specified and hex_value is wider, truncate
        bits.truncate(num_bits);
        log::warn!("High-order bits of the number are being truncated to fit the specified size.");
    } else if (bits.len() < num_bits) {
        // - when size is specified and hex_value is narrower, fill high-order bits with 0
        bits.zext(num_bits);
    }
    
    // TODO: figure out how the sign affects high-order bits
    // TODO: do something with `signed` variable
    if ast.signed {
        log::warn!("Signed hex number parsing is not yet fully supported. Results might not be correct.");
    }

    bits
}

pub fn parse_verilog_number(input: &str) -> Result<Bits, pest::error::Error<Rule>> {
    // let pairs = VerilogNumberParser::parse(Rule::number, input)?.next().unwrap().into_inner();
    let number = VerilogNumberParser::parse(Rule::number, input)?.next().unwrap();
    let number = number.into_inner().next().unwrap();
    // println!("number: {:#?}", number);

    let ast = match number.as_rule() {
        Rule::decimal_number => todo!(),
        Rule::binary_number => todo!(),
        Rule::octal_number => todo!(),
        Rule::hex_number => parse_hex_number(number),
        _ => unreachable!(),
    };

    log::debug!("ast: {ast:#?}");

    let bits = verilog_number_to_bits(ast);

    Ok(bits)
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_verilog_number() {
        let bits = parse_verilog_number("8'hAB").unwrap();
        assert_eq!(bits.to_string(), "10101011");
    }

    #[test]
    fn test_parsed_hex_number_size() {
        fn test_size(input: &str, correct_size: usize) {
            let hex_number_rule = VerilogNumberParser::parse(Rule::number, input).unwrap().next().unwrap().into_inner().next().unwrap();
            let ast = parse_hex_number(hex_number_rule);
            assert_eq!(ast.size(), correct_size);
        }
        test_size("'h0", 1);
        test_size("'h1", 1);
        test_size("'h2", 2);
        test_size("'h5", 3);
        test_size("'h0F", 4);
        test_size("'h1F", 5);
        test_size("'h2F", 6);
        test_size("'h4F", 7);
        test_size("'h8F", 8);
        test_size("'h10", 5);
        test_size("'h010", 5);
        test_size("9'h010", 9);
        test_size("'h0ab", 8);
        test_size("11'h0ab", 11);
        test_size("'h2ab", 10);
        test_size("32'h0", 32);
        test_size("1024'h0", 1024);

        // TODO: test size inference of signed hex numbers
    }

    #[test]
    fn test_bits_from_verilog_hex() {
        fn test_bit_parsing(input: &str, correct_str: &str) {
            let bits = parse_verilog_number(input).unwrap();
            assert_eq!(bits.to_string(), correct_str);
        }
        test_bit_parsing("'h0", "0");
        test_bit_parsing("'h1", "1");
        test_bit_parsing("'h6", "110");
        test_bit_parsing("5'h6", "00110");
        test_bit_parsing("11'hF03", "11100000011");
        test_bit_parsing("13'hF03", "0111100000011");
        test_bit_parsing("32'h1234_5678", "00010010001101000101011001111000");
        
        // TODO: test bit parsing of signed hex numbers
    }
}