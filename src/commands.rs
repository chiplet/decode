use std::process::exit;

use crate::{binary_value::BinaryValue, dtypes::DataType};

pub fn explain(value_str: &str, dtype: DataType) {
    // parse value string representation into internal format
    let binary_value = match BinaryValue::from(value_str) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Could not parse binary value from input: {}", value_str);
            eprintln!("{:#?}", e);
            exit(-1);
        }
    };

    log::debug!("Parsed binary value from {value_str}:\n{binary_value:?}");

    match dtype {
        DataType::F32 => explain_f32(),
        DataType::F64 => todo!(),
    }
}

fn explain_f32() {}