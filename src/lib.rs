use bitvec::prelude::*;
use pest::iterators::{Pair, Pairs};

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser {
    #[derive(Parser)]
    #[grammar = "value.pest"]
    pub struct ValueParser;
}

#[derive(Debug)]
pub enum Signedness {
    Unsigned,
    Signed,
}

fn push_bits(bv: &mut BitVec<u8, Lsb0>, bits: &[bool]) {
    for b in bits.iter().rev() {
        bv.push(*b);
    }
}

/// Binary value of arbitrary width that can be unsigned or signed
#[derive(Debug)]
pub struct BinaryValue {
    signedness: Signedness,
    width: usize,
    bits: BitVec::<u8, Lsb0>,
}
impl BinaryValue {
    pub fn from(parseResult: Pairs<parser::Rule>) -> Self {
        let syntaxKind = parseResult.peek().unwrap().into_inner().peek().unwrap();
        println!("{:?}", syntaxKind);
        let mut bits = BitVec::<u8, Lsb0>::new();
        match syntaxKind.as_rule() {
            parser::Rule::generic_hex_number => {
                // TODO: extract hex body parsing function
                let hex_body = syntaxKind
                    .into_inner()
                    .find_map(|x| match x.as_rule() {
                        parser::Rule::generic_hex_value => Some(x),
                        _ => None,
                    })
                    .unwrap().as_span();
                for c in hex_body.as_str().chars().rev() {
                    match c {
                        '0' => push_bits(&mut bits,       &[false, false, false, false]),
                        '1' => push_bits(&mut bits,       &[false, false, false, true]),
                        '2' => push_bits(&mut bits,       &[false, false, true,  false]),
                        '3' => push_bits(&mut bits,       &[false, false, true,  true]),
                        '4' => push_bits(&mut bits,       &[false, true,  false, false]),
                        '5' => push_bits(&mut bits,       &[false, true,  false, true]),
                        '6' => push_bits(&mut bits,       &[false, true,  true,  false]),
                        '7' => push_bits(&mut bits,       &[false, true,  true,  true]),
                        '8' => push_bits(&mut bits,       &[true,  false, false, false]),
                        '9' => push_bits(&mut bits,       &[true,  false, false, true]),
                        'a' | 'A' => push_bits(&mut bits, &[true,  false, true,  false]),
                        'b' | 'B' => push_bits(&mut bits, &[true,  false, true,  true]),
                        'c' | 'C' => push_bits(&mut bits, &[true,  true,  false, false]),
                        'd' | 'D' => push_bits(&mut bits, &[true,  true,  false, true]),
                        'e' | 'E' => push_bits(&mut bits, &[true,  true,  true,  false]),
                        'f' | 'F' => push_bits(&mut bits, &[true,  true,  true,  true]),
                        _ => panic!("should never happen!"),
                    }
                }
                println!("bits {:?}", bits);
            }
            _ => unimplemented!("No BinaryValue parser for {:?}", syntaxKind.as_rule()),
        }
        BinaryValue {
            signedness: Signedness::Unsigned,
            width: bits.len(),
            bits
        }
    }

    pub fn print(&self) {
        for b in self.bits.iter().rev().take(self.width).map(|b| match *b {
            false => 0,
            true => 1,
        }) {
            print!("{}", b);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
