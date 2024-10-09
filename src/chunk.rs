use byteorder::{WriteBytesExt, BE};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

use crate::to_bytes::ToBytes;

#[derive(Debug, Clone)]
struct MapChunk {
    chunk_x: i32,
    chunk_y: i16,
    chunk_z: i32,
    size_x: u8,
    size_y: u8,
    size_z: u8,
    compressed_data: BlockArray,
}

impl MapChunk {
    fn new(chunk_x: i32, chunk_z: i32, compressed_data: BlockArray) -> Self {
        Self {
            chunk_x,
            chunk_y: 0,
            chunk_z,
            size_x: 15,
            size_y: 127,
            size_z: 15,
            compressed_data,
        }
    }
}

#[derive(Debug, Clone)]
struct BlockArray {
    compressed_size: i32,
    compressed_data: Vec<u8>,
}

impl BlockArray {
    fn new_air() -> Self {
        let mut output_vec = Vec::new();

        for _ in 0..(16 * 127 * 15) {
            output_vec.push(0);
        }
        for _ in 0..(16 * 127 * 15) / 2 {
            output_vec.push(0);
        }
        for _ in 0..(16 * 127 * 15) / 2 {
            output_vec.push(0);
        }
        for _ in 0..(16 * 127 * 15) / 2 {
            output_vec.push(0);
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write(&output_vec).unwrap();

        Self {
            compressed_size: 1,
            compressed_data: encoder.finish().unwrap(),
        }
    }
}

#[repr(u8)]
enum BlockType {
    Air,
    Stone,
    Grass,
    Dirt,
    Cobblestone,
    Planks,
    Sapling,
    Bedrock,
}

impl ToBytes for MapChunk {
    type Bytes = Vec<u8>;

    fn to_bytes(self) -> Self::Bytes {
        let mut buffer = Vec::new();
        buffer.write_i32::<BE>(self.chunk_x).unwrap();
        buffer.write_i16::<BE>(self.chunk_y).unwrap();
        buffer.write_i32::<BE>(self.chunk_z).unwrap();
        buffer.write_u8(self.size_x).unwrap();
        buffer.write_u8(self.size_y).unwrap();
        buffer.write_u8(self.size_z).unwrap();

        buffer
    }
}

#[repr(C)]
pub struct PreChunk {
    pub x_coord: i32,
    pub z_coord: i32,
    pub mode: bool, // True to load, False to unload
}

impl ToBytes for PreChunk {
    type Bytes = [u8; 9];

    fn to_bytes(self) -> Self::Bytes {
        let mut buffer = Vec::new();
        buffer.write_i32::<BE>(self.x_coord).unwrap();
        buffer.write_i32::<BE>(self.z_coord).unwrap();
        buffer.write_u8(self.mode as u8).unwrap();

        buffer.try_into().unwrap()
    }
}
