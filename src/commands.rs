use std::process::exit;

use crate::{binary_value::BinaryValue, dtypes::DataType, frontend::verilog::parse_verilog_number};

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

    log::debug!("Parsed bits from {value_str}:\n{bits:}");

    match dtype {
        DataType::F32 => explain_f32(),
        DataType::F64 => todo!(),
    }
}

fn explain_f32() {
    todo!()
}
