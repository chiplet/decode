use std::process::exit;

use clap::Parser as ClapParser;
use decode::parser::*;
use pest::Parser as PestParser;

#[derive(ClapParser, Debug)]
struct Arguments {
    value: String,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);

    let value = match decode::parser::ValueParser::parse(Rule::value, &args.value) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Could not parse binary value from input: {}", args.value);
            eprintln!("{:#?}", e);
            exit(-1);
        }
    };
    println!("{:#?}", &value);

    let binaryValue = decode::BinaryValue::from(value);
    binaryValue.print();
}
