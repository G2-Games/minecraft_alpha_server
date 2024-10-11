use byteorder::{WriteBytesExt, BE};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use num_derive::FromPrimitive;
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
    blocks: Vec<u8>,
    metadata: Vec<u8>,
    block_light: Vec<u8>,
    sky_light: Vec<u8>,
}

const CHUNK_WIDTH_X: usize = 16;
const CHUNK_WIDTH_Z: usize = 16;
const CHUNK_HEIGHT_Y: usize = 128;
const CHUNK_TOTAL_BLOCKS: usize = CHUNK_WIDTH_X * CHUNK_WIDTH_Z * CHUNK_HEIGHT_Y;

impl BlockArray {
    fn compress(self) -> Vec<u8> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write(&self.blocks).unwrap();
        encoder.write(&self.metadata).unwrap();
        encoder.write(&self.block_light).unwrap();
        encoder.write(&self.sky_light).unwrap();
        let output_buf = encoder.finish().unwrap();

        output_buf
    }

    pub fn new_air() -> Self {
        Self {
            blocks: vec![0; CHUNK_TOTAL_BLOCKS],
            metadata: vec![0; CHUNK_TOTAL_BLOCKS / 2],
            block_light: vec![0; CHUNK_TOTAL_BLOCKS / 2],
            sky_light: vec![0xFF; CHUNK_TOTAL_BLOCKS / 2],
        }
    }

    pub fn new_superflat() -> Self {
        let mut blocks = vec![0; CHUNK_TOTAL_BLOCKS];
        for y in 0..CHUNK_HEIGHT_Y {
            for x in 0..CHUNK_WIDTH_X {
                for z in 0..CHUNK_WIDTH_Z {
                    let pos = y + (z * (CHUNK_HEIGHT_Y)) + (x * (CHUNK_HEIGHT_Y) * (CHUNK_WIDTH_X));
                    if y == 7 {
                        blocks[pos] = BlockType::Grass as u8;
                    } else if y > 0 && y < 7 {
                        blocks[pos] = BlockType::Dirt as u8;
                    } else if y == 0 {
                        blocks[pos] = BlockType::Bedrock as u8;
                    } else {
                        blocks[pos] = BlockType::Air as u8;
                    }
                }
            }
        }

        Self {
            blocks,
            metadata: vec![0xFF; CHUNK_TOTAL_BLOCKS / 2],
            block_light: vec![0; CHUNK_TOTAL_BLOCKS / 2],
            sky_light: vec![0xFF; CHUNK_TOTAL_BLOCKS / 2],
        }
    }
}

#[repr(i16)]
#[derive(Debug, Clone, Copy)]
#[derive(FromPrimitive)]
pub enum BlockType {
    None = -1,
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

        let block_buf = self.compressed_data.compress();
        buffer.write_i32::<BE>(block_buf.len() as i32).unwrap();
        buffer.write_all(&block_buf).unwrap();

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
