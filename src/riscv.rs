use bitvec::prelude::*;
use crate::binary_value::BinaryValue;

/// Meaning associated with a bitfield
struct Field<'a> {
    source: &'a BitSlice<u8>,
    name: String,
    description: String,
}

// TODO: use interpretation as result/option-like container for fields
enum Interpretation<'a> {
    Decoded(Field<'a>),
    Defer(&'a BitSlice<u8>), // defer interpretation to the next decode pass
}

trait Decode<'a> {
    fn decode(bs: &BitSlice) -> Vec<Interpretation<'a>>;
}

/// decode BinaryValue as itself
impl Decode for BinaryValue {

}

trait RVInstructionOverlay: Decode {
    padding: Defer,
    opcode: OpcodeDecoder,
}

// #[repr(u8)]
// pub enum Opcode {
//     LOAD = 0b00000_11,
//     LOAD_FP = 0b00001_11,
//     custom_0 = 0b00010_11,
//     MISC_MEM = 0b00011_11,
//     OP_IMM = 0b00100_11,
//     AUIPC = 0b00101_11,
//     OP_IMM_32 = 0b00110_11,
//     STORE = 0b01000_11,
//     STORE_FP = 0b01001_11,
//     custom_1 = 0b01010_11,
//     AMO = 0b01011_11,
//     OP = 0b01100_11,
//     LUI = 0b01101_11,
//     OP_32 = 0b01110_11,
//     MADD = 0b10000_11,
//     MSUB = 0b10001_11,
//     NMSUB = 0b10010_11,
//     NMADD = 0b10011_11,
//     OP_FP = 0b10100_11,
//     reserved = 0b10101_11,
//     custom_2__rv128 = 0b10110_11,
//     BRANCH = 0b11000_11,
//     JALR = 0b11001_11,
//     reserved = 0b11010_11,
//     JAL = 0b11011_11,
//     SYSTEM = 0b11100_11,
//     reserved = 0b11101_11,
//     custom_3__rv128 = 0b11110_11,
// }
// pub enum Instruction {
//     BEQ,
//     BNE,
//     BLT,
//     BGE,
//     BLTU,
//     BGEU,
//     JALR,
//     JAL,
//     LUI,
//     AUIPC,
//     ADDI,
//     SLLI,
//     SRLI,
//     SRAI,
//     SLTI,
//     SLTIU,
//     XORI,
//     ORI,
//     ANDI,
//     ADD,
//     SUB,
//     SLL,
//     SLT,
//     SLTU,
//     XOR,
//     SRL,
//     SRA,
//     OR,
//     AND,
//     LB,
//     LH,
//     LW,
//     LBU,
//     LHU,
//     SB,
//     SH,
//     SW,
// }


struct Opcode {

}

pub struct RISCVInstruction {

}
impl RISCVInstruction {
    pub fn from(binval: &BinaryValue) -> Result<Self, String> {
        match binval.len() {
            16 => {
                // compressed
                unimplemented!();
            }
            32 => {
                // RV32I, RV64I
                let opcode = &binval.bits[0..7].load::<u8>();
                let funct3 = &binval.bits[12..15].load::<u8>();
                let funct7 = &binval.bits[25..32].load::<u8>();
                println!("opcode {:#?}", opcode);
                
                match opcode {
                    // TODO: how to match BitSlice values?
                    0b00000_11 => { // LOAD
                        todo!()
                    },
                    0b00001_11 => { // LOAD-FP
                        todo!()
                    },
                    0b01101_11 => panic!("LUI"),
                    0b00101_11 => panic!("AUIPC"),
                    128.. => unreachable!(),
                    _ => todo!(),
                }
            },
            _ => {
                Err(String::from(format!("Invalid number of bits for a RISC-V instruction: {}", binval.len())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_value::*;

    // TODO: create assert macro for BinaryValue types
    #[test]
    fn binary_decode_works() {
        let binval = BinaryValue::from("0x1234").unwrap();

    }

    #[test]
    fn auipc_decode_works() {
        let result = BinaryValue::from("0x20000197").unwrap();

    }
}