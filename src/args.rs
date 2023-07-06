use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(Parser)]
pub struct Encode {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}

#[derive(Parser)]
pub struct Decode {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct Remove {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct Print {
    pub file_path: String,
}
