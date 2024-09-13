use clap::{Parser, Subcommand, ValueEnum};
use decode::{binary_value::*, commands, dtypes::DataType, riscv::RISCVInstruction};
use std::process::exit;


#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Explain {
        number: String,

        #[arg(long)]
        dtype: DataType,
    }
}



fn main() {
    env_logger::init();

    let args = Args::parse();
    log::debug!("{:?}", args);    

    match args.command {
        Commands::Explain { number, dtype } => commands::explain(&number, dtype),
    }
    
}
