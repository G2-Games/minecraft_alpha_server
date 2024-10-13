use byteorder::{ReadBytesExt, WriteBytesExt, BE};

use crate::mcstring::{MCString, WriteMCString, ReadMCString};

use super::Packet;

#[derive(Debug, Clone)]
pub struct Packet1Login {
    pub username: MCString,
    pub password: MCString,
    pub protocol_version: i32,
    pub world_seed: i64,
    pub dimension: i8,
}

impl Packet1Login {
    pub fn new(protocol_version: i32, world_seed: i64, dimension: i8) -> Self {
        Self {
            username: MCString::default(),
            password: MCString::default(),
            protocol_version,
            world_seed,
            dimension,
        }
    }
}

impl Packet for Packet1Login {
    fn read_from<R: std::io::Read>(input: &mut R) -> Result<Self, std::io::Error> {
        let protocol_version = input.read_i32::<BE>()?;
        let username = input.read_mcstring()?.into();
        let password = input.read_mcstring()?.into();
        let world_seed = input.read_i64::<BE>()?;
        let dimension = input.read_i8()?;

        Ok(Packet1Login {
            username,
            password,
            protocol_version,
            world_seed,
            dimension,
        })
    }

    fn write_into<W: std::io::Write>(&self, output: &mut W) -> Result<(), std::io::Error> {
        output.write_i32::<BE>(self.protocol_version).unwrap();
        output.write_mcstring(&self.username).unwrap();
        output.write_mcstring(&self.password).unwrap();
        output.write_i64::<BE>(self.world_seed).unwrap();
        output.write_i8(self.dimension).unwrap();

        Ok(())
    }

    fn size(&self) -> usize {
        4 + self.username.len() + self.password.len() + 8 + 1
    }
}
