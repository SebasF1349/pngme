use std::io::prelude::*;
use std::{fs::File, str::FromStr};

use args::{Cli, Commands};
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => {
            let mut png = read_file(&file_path)?;
            png.append_chunk(Chunk::new(
                ChunkType::from_str(&chunk_type)?,
                message.as_bytes().to_vec(),
            ));
            let new_path = match output_file {
                Some(file) => file,
                None => file_path,
            };
            let mut new_file = File::create(new_path)?;
            new_file.write_all(png.as_bytes().as_slice())?;
            println!("Chunk appended");
            Ok(())
        }
        Commands::Decode {
            file_path,
            chunk_type,
        } => {
            let png = read_file(&file_path)?;
            let chunk = png.chunk_by_type(&chunk_type);
            match chunk {
                Some(chunk) => println!("Message: {}", chunk.data_as_string().unwrap()),
                None => println!("Chunk not found"),
            }
            Ok(())
        }
        Commands::Remove {
            file_path,
            chunk_type,
        } => {
            let mut png = read_file(&file_path)?;
            png.remove_chunk(&chunk_type)?;
            let mut new_file = File::create(file_path)?;
            new_file.write_all(png.as_bytes().as_slice())?;
            println!("Chunk removed");
            Ok(())
        }
        Commands::Print { file_path } => {
            let png = read_file(&file_path)?;
            println!("{}", png);
            Ok(())
        }
    }
}

fn read_file(file_path: &str) -> Result<Png> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let png = Png::try_from(&buffer[..])?;
    Ok(png)
}
