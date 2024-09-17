use std::{collections::HashMap, process::exit};

use crate::{bits::Bits, decoder::{dtypes::DataType, ieee754::F32Decoder, riscv::RISCVDecoder}, frontend::verilog::parse_verilog_number, Fields, IdxRange};
use std::fmt;

pub fn explain(value_str: &str, dtype: DataType) {
    // parse value string representation into internal format
    let bits = match parse_verilog_number(value_str) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Could not parse binary value from input: {}", value_str);
            eprintln!("{:#?}", e);
            exit(-1);
        }
    };

    log::info!("Parsed bits from {value_str}:\n{bits:}");

    let decoder: Box<dyn Fields> = match dtype {
        DataType::F32 => Box::new(F32Decoder),
        DataType::F64 => todo!(),
        DataType::RISCV => Box::new(RISCVDecoder),
    };

    // print fields
    for (name, range) in decoder.fields(&bits) {
        println!("{}: {:?}", name, range);
    }
}

fn explain_f32(bits: &Bits) {
    let f32_decoder = F32Decoder;
    let fields = f32_decoder.fields(bits);
    println!("fields:  {fields:#?}");
}
