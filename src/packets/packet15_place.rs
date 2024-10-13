use crate::{blocks_items::{BlockItem, BlockItemID, ItemStack}, player::Direction};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use num_traits::FromPrimitive;

use super::Packet;

#[derive(Debug, Clone, Copy)]
pub struct Packet15Place {
    id: BlockItem,
    x_position: i32,
    y_position: u8,
    z_position: i32,
    direction: u8,
    amount: Option<u8>,
    health: Option<i16>,
}

impl Packet for Packet15Place {
    fn read_from<R: std::io::Read>(input: &mut R) -> Result<Self, std::io::Error> {
        let id = BlockItem::from_id(input.read_i16::<BE>()?);
        Ok(Self {
            id,
            x_position: input.read_i32::<BE>()?,
            y_position: input.read_u8()?,
            z_position: input.read_i32::<BE>()?,
            direction: input.read_u8()?,
            amount: if id.id() <= 0 { None } else { Some(input.read_u8()?) },
            health: if id.id() <= 0 { None } else { Some(input.read_i16::<BE>()?) }
        })
    }

    fn write_into<W: std::io::Write>(&self, output: &mut W) -> Result<(), std::io::Error> {
        unimplemented!()
    }

    fn size(&self) -> usize {
        todo!()
    }
}
