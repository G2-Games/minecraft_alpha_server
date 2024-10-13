use std::io::{self, Read, Write};

pub mod packet1_login;
pub mod packet15_place;
pub mod packet101;

/// A packet for communicating across the network.
pub trait Packet
    where Self: Sized
{
    /// Read the packet in from a stream
    fn read_from<R: Read>(input: &mut R) -> Result<Self, io::Error>;

    /// Write the packet out to a stream
    fn write_into<W: Write>(&self, output: &mut W) -> Result<(), io::Error>;

    /// The size of the packet in bytes
    fn size(&self) -> usize;
}
