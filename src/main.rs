mod utils;
mod to_bytes;
mod chunk;

use std::{io::{self, Write}, net::{TcpListener, TcpStream}};

use chunk::PreChunk;
use log::{info, warn};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use to_bytes::ToBytes;
use utils::{MCString, ReadMCString, WriteMCString};


fn main() {
    colog::default_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

    for mut connection in listener.incoming().filter_map(|c| c.ok()) {
        info!("Connected to client @ {}", connection.peer_addr().unwrap());
        while let Some(cmd) = connection.read_u8().ok() {
            let command = Command::try_from(cmd);
            info!("COMMAND: {command:?}");
            handle_command(&mut connection, command.unwrap()).unwrap();

        }
        warn!("Lost connection to client");
    }
}

fn handle_command(connection: &mut TcpStream, command: Command) -> Result<(), io::Error> {
    match command {
        Command::KeepAlive => todo!(),
        Command::Handshake => {
            let username = connection.read_mcstring()?;
            info!("Handshake: {username}");

            connection.write_u8(0x02)?;
            connection.write_mcstring(&MCString::try_from("-").unwrap())?;
        },
        Command::Login => {
            info!("---");
            info!("Initiating login");
            let protocol_version = connection.read_u32::<BE>()?;
            let username = connection.read_mcstring()?;
            let _password = connection.read_mcstring()?;
            let _map_seed = connection.read_i64::<BE>()?;
            let _dimension = connection.read_i8()?;

            info!("Protocol Version: {protocol_version}");
            info!("Username: {username}");

            let login_packet = ServerLoginPacket {
                entity_id: 1,
                unknown1: MCString::default(),
                unknown2: MCString::default(),
                map_seed: 1715505462032542147,
                dimension: 0,
            };
            login_packet.write_into(connection)?;

            info!("Responded to auth request");

            let prechunk = PreChunk {
                x_coord: 0,
                z_coord: 0,
                mode: true,
            };

            connection.write_u8(Command::PreChunk as u8)?;
            connection.write_all(&prechunk.to_bytes())?;
        },
        _ => unimplemented!("This command is probably `Server -> Client` only")
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Command {
    KeepAlive = 0x00,
    Login = 0x01,
    Handshake = 0x02,
    PreChunk = 0x32,
    ChunkData = 0x33,
    Kick = 0xFF,
}

#[derive(Debug, Clone, Copy)]
struct CommandError {
    _id: u8,
}

impl TryFrom<u8> for Command {
    type Error = CommandError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => Self::KeepAlive,
            0x01 => Self::Login,
            0x02 => Self::Handshake,
            0x32 => Self::PreChunk,
            0x33 => Self::ChunkData,
            0xFF => Self::Kick,
            v => return Err(CommandError{
                _id: v,
            }),
        })
    }
}

struct ServerLoginPacket {
    entity_id: i32,
    unknown1: MCString,
    unknown2: MCString,
    map_seed: i64,
    dimension: i8,
}

impl ServerLoginPacket {
    fn write_into<W: Write>(&self, stream: &mut W) -> Result<(), io::Error> {
        stream.write_i32::<BE>(self.entity_id)?;
        stream.write_mcstring(&self.unknown1)?;
        stream.write_mcstring(&self.unknown2)?;
        stream.write_i64::<BE>(self.map_seed)?;
        stream.write_i8(self.dimension)?;

        Ok(())
    }
}
