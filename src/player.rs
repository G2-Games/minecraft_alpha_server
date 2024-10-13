use num_derive::FromPrimitive;

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
