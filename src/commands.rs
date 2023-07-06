use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};
use std::io::prelude::*;
use std::{fs::File, str::FromStr};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn encode(
    file_path: String,
    chunk_type: String,
    message: String,
    output_file: Option<String>,
) -> Result<()> {
    let mut png: Png = std::fs::read(&file_path)?.as_slice().try_into()?;
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

pub fn decode(file_path: String, chunk_type: String) -> Result<()> {
    let png: Png = std::fs::read(file_path)?.as_slice().try_into()?;
    let chunk = png.chunk_by_type(&chunk_type);
    match chunk {
        Some(chunk) => println!("Message: {}", chunk.data_as_string().unwrap()),
        None => println!("Chunk not found"),
    }
    Ok(())
}

pub fn remove(file_path: String, chunk_type: String) -> Result<()> {
    let mut png: Png = std::fs::read(&file_path)?.as_slice().try_into()?;
    png.remove_chunk(&chunk_type)?;
    let mut new_file = File::create(file_path)?;
    new_file.write_all(png.as_bytes().as_slice())?;
    println!("Chunk removed");
    Ok(())
}

pub fn print(file_path: String) -> Result<()> {
    let png: Png = std::fs::read(file_path)?.as_slice().try_into()?;
    println!("{}", png);
    Ok(())
}
