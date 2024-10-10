use byteorder::{WriteBytesExt, BE};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

use crate::byte_ops::ToBytes;

#[derive(Debug, Clone)]
pub struct MapChunk {
    chunk_x: i32,
    chunk_y: i16,
    chunk_z: i32,
    size_x: u8,
    size_y: u8,
    size_z: u8,
    compressed_data: BlockArray,
}

impl MapChunk {
    pub fn new(chunk_x: i32, chunk_z: i32, compressed_data: BlockArray) -> Self {
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
pub struct BlockArray {
    compressed_size: i32,
    compressed_data: Vec<u8>,
}

impl BlockArray {
    pub fn new_air() -> Self {
        let mut block_vec = Vec::new();

        for _ in 0..(16 * 127 * 15) {
            block_vec.push(0);
        }
        for _ in 0..(16 * 127 * 15) / 2 {
            block_vec.push(0);
        }
        for _ in 0..(16 * 127 * 15) / 2 {
            block_vec.push(0);
        }
        for _ in 0..(16 * 127 * 15) / 2 {
            block_vec.push(0);
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write(&block_vec).unwrap();
        let output_buf = encoder.finish().unwrap();

        Self {
            compressed_size: output_buf.len() as i32,
            compressed_data: output_buf,
        }
    }

    pub fn new_superflat() -> Self {
        let mut block_vec = vec![0; 16 * 16 * 128];

        for x in 0..16 {
            for y in 0..128 {
                for z in 0..16 {
                    let pos = y + (z * (128)) + (x * (128) * (16));
                    if y == 7 {
                        block_vec[pos] = BlockType::Grass as u8;
                    } else if y > 0 && y < 7 {
                        block_vec[pos] = BlockType::Dirt as u8;
                    } else if y == 0 {
                        block_vec[pos] = BlockType::Bedrock as u8;
                    } else {
                        block_vec[pos] = 0;
                    }
                }
            }
        }
        for _ in 0..(16 * 128 * 16) / 2 {
            block_vec.push(0);
        }
        for _ in 0..(16 * 128 * 16) / 2 {
            block_vec.push(0);
        }
        for _ in 0..(16 * 128 * 16) / 2 {
            block_vec.push(0xFF);
        }

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write(&block_vec).unwrap();
        let output_buf = encoder.finish().unwrap();

        Self {
            compressed_size: output_buf.len() as i32,
            compressed_data: output_buf,
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

        buffer.write_i32::<BE>(self.compressed_data.compressed_size).unwrap();
        buffer.write_all(&self.compressed_data.compressed_data).unwrap();

        buffer
    }
}

#[repr(C)]
pub struct PreChunk {
    pub x_coord: i32,
    pub z_coord: i32,
    pub mode: bool, // True to load, False to unload
}

impl PreChunk {
    pub fn new_load(x_coord: i32, z_coord: i32) -> Self {
        Self {
            x_coord,
            z_coord,
            mode: true
        }
    }

    pub fn new_unload(x_coord: i32, z_coord: i32) -> Self {
        Self {
            x_coord,
            z_coord,
            mode: true
        }
    }
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
