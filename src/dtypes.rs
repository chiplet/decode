use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
/// List of supported data types for decoding
pub enum DataType {
    F32,
    F64,
}