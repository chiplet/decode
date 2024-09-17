use std::collections::HashMap;

use crate::{bits::{Bit, Bits}, Fields, IdxRange};

pub enum Opcode {
    LOAD,
    LOAD_FP,
    custom_0,
    MISC_MEM,
    OP_IMM,
    AUIPC,
    OP_IMM_32,
    STORE,
    STORE_FP,
    custom_1,
    AMO,
    OP,
    LUI,
    OP_32,
    MADD,
    MSUB,
    NMSUB,
    NMADD,
    OP_FP,
    custom_2__rv128,
    BRANCH,
    JALR,
    JAL,
    SYSTEM,
    custom_3__rv128,
    reserved,
}

pub enum Instruction {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    JALR,
    JAL,
    LUI,
    AUIPC,
    ADDI,
    SLLI,
    SRLI,
    SRAI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
}


pub struct RISCVDecoder;
impl<'a> Fields<'a> for RISCVDecoder {
    fn fields(&self, bits: &'a Bits) -> HashMap<&str, Vec<(&'a [Bit], IdxRange)>> {
        match bits.len() {
            16 => {
                // compressed
                unimplemented!();
            },
            32 => {
                // RV32I
                let mut riscv_fields = HashMap::new();
                riscv_fields.insert("opcode", vec![(&bits[0..=6], IdxRange::new(6, 0))]);
                riscv_fields
            },
            64 => {
                // RV64I
                unimplemented!();
            },
            _ => panic!("Invalid number of bits for a RISC-V instruction: {}", bits.len()),
        }
    }
}
