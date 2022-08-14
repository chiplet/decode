use bitvec::prelude::*;
use crate::binary_value::BinaryValue;

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

/// Struct containing all information required to display the associated
/// RISC-V instruction in a human readable format.
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
                let opcode = &binval.bits[0..7];
                println!("opcode {:#?}", opcode);
                
                match opcode {
                    // TODO: how to match BitSlice values?
                    _ => todo!(),
                }
            },
            _ => {
                Err(String::from(format!("Invalid number of bits for a RISC-V instruction: {}", binval.len())))
            }
        }
    }
}