use std::{fmt::Display, io::{self, Read, Write}};

use byteorder::{ReadBytesExt, WriteBytesExt, BE};

#[derive(Debug, Default, Clone)]
pub struct MCString {
    len: u16,
    chars: Vec<u8>,
}

impl TryFrom<&str> for MCString {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > u16::MAX as usize {
            return Err(())
        }

        Ok(Self {
            len: value.len() as u16,
            chars: value.as_bytes().to_vec(),
        })
    }
}

impl Display for MCString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.chars.clone()).unwrap())
    }
}

pub trait ReadMCString: io::Read {
    #[inline]
    fn read_mcstring(&mut self) -> Result<MCString, io::Error> {
        let len = self.read_u16::<BE>()?;
        let mut string = vec![0; len as usize];
        self.read_exact(&mut string)?;

        Ok(MCString {
            len,
            chars: string,
        })
    }
}

impl<R: Read> ReadMCString for R {}

pub trait WriteMCString: io::Write {
    #[inline]
    fn write_mcstring(&mut self, string: &MCString) -> Result<(), io::Error> {
        self.write_u16::<BE>(string.len)?;

        if string.len != 0 {
            self.write_all(&string.chars)?;
        }

        Ok(())
    }
}

impl<W: Write> WriteMCString for W {}
