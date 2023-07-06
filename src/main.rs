use args::{Cli, Commands, Decode, Encode, Print, Remove};
use clap::Parser;
use commands::{decode, encode, print, remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Encode(Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        }) => encode(file_path, chunk_type, message, output_file),
        Commands::Decode(Decode {
            file_path,
            chunk_type,
        }) => decode(file_path, chunk_type),
        Commands::Remove(Remove {
            file_path,
            chunk_type,
        }) => remove(file_path, chunk_type),
        Commands::Print(Print { file_path }) => print(file_path),
    }
}
