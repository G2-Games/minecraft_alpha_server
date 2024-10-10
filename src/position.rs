use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt, BE};

use crate::byte_ops::ToBytes;

#[derive(Debug, Clone, Copy)]
pub struct PlayerPositionAndLook {
    pub position: PlayerPosition,
    pub look: PlayerLook,
}

impl ToBytes for PlayerPositionAndLook {
    type Bytes = Vec<u8>;

    fn to_bytes(self) -> Self::Bytes {
        let mut out_buf = Vec::new();
        out_buf.write_f64::<BE>(self.position.position_x).unwrap();
        out_buf.write_f64::<BE>(self.position.position_y).unwrap();
        out_buf.write_f64::<BE>(self.position.stance).unwrap();
        out_buf.write_f64::<BE>(self.position.position_z).unwrap();
        out_buf.write_f32::<BE>(self.look.yaw).unwrap();
        out_buf.write_f32::<BE>(self.look.pitch).unwrap();
        out_buf.write_u8(true as u8).unwrap();

        out_buf
    }
}

impl PlayerPositionAndLook {
    pub fn from_bytes<R: Read>(stream: &mut R) -> Self {
        let position_x = stream.read_f64::<BE>().unwrap();
        let position_y = stream.read_f64::<BE>().unwrap();
        let stance = stream.read_f64::<BE>().unwrap();
        let position_z = stream.read_f64::<BE>().unwrap();

        let yaw = stream.read_f32::<BE>().unwrap();
        let pitch = stream.read_f32::<BE>().unwrap();

        let _on_ground = stream.read_u8().unwrap();

        Self {
            position: PlayerPosition {
                position_x,
                stance,
                position_y,
                position_z,
            },
            look: PlayerLook {
                yaw,
                pitch,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerPosition {
    pub position_x: f64,
    pub position_y: f64,
    pub stance: f64,
    pub position_z: f64,
}

impl ToBytes for PlayerPosition {
    type Bytes = Vec<u8>;

    fn to_bytes(self) -> Self::Bytes {
        let mut out_buf = Vec::new();
        out_buf.write_f64::<BE>(self.position_x).unwrap();
        out_buf.write_f64::<BE>(self.position_y).unwrap();
        out_buf.write_f64::<BE>(self.stance).unwrap();
        out_buf.write_f64::<BE>(self.position_z).unwrap();
        out_buf.write_u8(1).unwrap();

        out_buf
    }
}

impl PlayerPosition {
    pub fn from_bytes<R: Read>(stream: &mut R) -> Self {
        let position_x = stream.read_f64::<BE>().unwrap();
        let position_y = stream.read_f64::<BE>().unwrap();
        let stance = stream.read_f64::<BE>().unwrap();
        let position_z = stream.read_f64::<BE>().unwrap();

        let _on_ground = stream.read_u8().unwrap() != 0;

        Self {
            position_x,
            stance,
            position_y,
            position_z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
}

impl ToBytes for PlayerLook {
    type Bytes = Vec<u8>;

    fn to_bytes(self) -> Self::Bytes {
        let mut out_buf = Vec::new();
        out_buf.write_f32::<BE>(self.yaw).unwrap();
        out_buf.write_f32::<BE>(self.pitch).unwrap();
        out_buf.write_u8(1).unwrap();

        out_buf
    }
}

impl PlayerLook {
    pub fn from_bytes<R: Read>(stream: &mut R) -> Self {
        let yaw = stream.read_f32::<BE>().unwrap();
        let pitch = stream.read_f32::<BE>().unwrap();

        let _on_ground = stream.read_u8().unwrap() != 0;

        Self {
            yaw,
            pitch,
        }
    }
}
