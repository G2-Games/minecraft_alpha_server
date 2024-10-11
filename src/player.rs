use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::chunk::BlockType;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[derive(FromPrimitive)]
pub enum DiggingStatus {
    StartedDigging = 0,
    Digging = 1,
    StoppedDigging = 2,
    BlockBroken = 3,
}

/// The face of a block, a direction.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[derive(FromPrimitive)]
pub enum Direction {
    NegY = 0,
    PosY = 1,
    NegZ = 2,
    PosZ = 3,
    NegX = 4,
    PosX = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerDigging {
    status: DiggingStatus,
    position_x: i32,
    position_y: u8,
    position_z: i32,
    face: Direction,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerBlockPlacement {
    block_id: BlockType,
    position_x: i32,
    position_y: u8,
    position_z: i32,
    direction: Direction,
    //amount: u8,
    //health: u8,
}

impl PlayerBlockPlacement {
    pub fn from_bytes<R: Read>(stream: &mut R) -> Self {
        let block_id = BlockType::from_i16(stream.read_i16::<BE>().unwrap()).unwrap();
        let position_x = stream.read_i32::<BE>().unwrap();
        let position_y = stream.read_u8().unwrap();
        let position_z = stream.read_i32::<BE>().unwrap();
        let direction = Direction::from_u8(stream.read_u8().unwrap()).unwrap();

        Self {
            block_id,
            position_x,
            position_y,
            position_z,
            direction,
        }
    }
}
