use clap::Parser;
use decode::binary_value::*;
use std::process::exit;

#[derive(Parser, Debug)]
struct Arguments {
    value: String,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);

    let binaryValue = match BinaryValue::from(&args.value) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Could not parse binary value from input: {}", args.value);
            eprintln!("{:#?}", e);
            exit(-1);
        }
    };
    println!("{:#?}", &binaryValue);
    binaryValue.print();
}
