#![allow(unused_variables)]
#![allow(dead_code)]

use crate::chunk_type::{ChunkType, ChunkTypeError};
use crc::crc32::checksum_ieee;
use std::fmt;

pub struct Chunk {
    pub length: [u8; 4],
    pub c_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: [u8; 4],
}

#[derive(Debug)]
pub enum ChunkError {
    Invalid,
    Length,
    ChunkType(ChunkTypeError),
    NonString,
}

impl Chunk {
    pub fn calculate_crc(chunk_type: ChunkType, data: Vec<u8>) -> u32 {
        let crc_bytes: Vec<u8> = chunk_type.bytes().iter().chain(&data).copied().collect();
        checksum_ieee(&crc_bytes)
    }

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc_bytes: Vec<u8> = chunk_type.bytes().iter().chain(&data).copied().collect();
        let crc = checksum_ieee(&crc_bytes);

        Chunk {
            length: u32::try_from(data.len()).unwrap().to_be_bytes(),
            c_type: chunk_type,
            data,
            crc: crc.to_be_bytes(),
        }
    }
    pub fn length(&self) -> u32 {
        u32::from_be_bytes(self.length)
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.c_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        u32::from_be_bytes(self.crc)
    }
    pub fn data_as_string(&self) -> Result<String, ChunkError> {
        String::from_utf8(self.data.clone()).map_err(|err| ChunkError::NonString)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_data: Vec<u8> = self
            .length
            .iter()
            .chain(&self.c_type.bytes)
            .chain(&self.data)
            .chain(&self.crc)
            .copied()
            .collect();
        chunk_data
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;
    fn try_from(values: &[u8]) -> Result<Self, Self::Error> {
        if values.len() < 12 {
            return Err(ChunkError::Length);
        }

        let crc_pos = values.len() - 4;

        let mut length: [u8; 4] = [0; 4];
        let mut c_type: [u8; 4] = [0; 4];
        let mut data: Vec<u8> = vec![];
        let mut crc: [u8; 4] = [0; 4];

        for (pos, value) in values.iter().enumerate() {
            match pos {
                ..=3 => length[pos] = *value,
                4..=7 => c_type[pos - 4] = *value,
                _ => {
                    if pos < crc_pos {
                        data.push(*value);
                    } else {
                        crc[pos - crc_pos] = *value;
                    }
                }
            }
        }

        let c_type = match ChunkType::try_from(c_type) {
            Ok(ct) => ct,
            Err(err) => return Err(ChunkError::ChunkType(err)),
        };

        let new_chunk = Self::new(c_type, data);

        if new_chunk.length == length && new_chunk.crc == crc {
            Ok(new_chunk)
        } else {
            Err(ChunkError::Invalid)
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
