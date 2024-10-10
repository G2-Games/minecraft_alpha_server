mod utils;
mod byte_ops;
mod chunk;
mod position;

use std::{cell::LazyCell, io::{self, Write}, net::{TcpListener, TcpStream}};

use base16ct::lower::encode_string;
use chunk::{BlockArray, MapChunk, PreChunk};
use log::{info, warn};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use byte_ops::ToBytes;
use position::{PlayerLook, PlayerPosition, PlayerPositionAndLook};
use utils::{MCString, ReadMCString, WriteMCString};
use rand::random;

const CHUNKS: LazyCell<Vec<MapChunk>> = LazyCell::new(|| {
    let mut mapchunk = Vec::new();
    for i in -10..10 {
        for o in -10..10 {
            let x = i * 16;
            let z = o * 16;

            mapchunk.push(MapChunk::new(x, z, BlockArray::new_superflat()));
        }
    }

    mapchunk
});

fn main() {
    colog::default_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

    for mut connection in listener.incoming().filter_map(|c| c.ok()) {
        info!("Connected to client @ {}", connection.peer_addr().unwrap());
        while let Some(cmd) = connection.read_u8().ok() {
            let command = Command::from_u8(cmd);
            if command.is_none() {
                info!("COMMAND: {command:?} (0x{cmd:02X?})");
            }
            handle_command(&mut connection, command.unwrap()).unwrap();
        }
        warn!("Lost connection to client");
    }
}

fn handle_command(mut connection: &mut TcpStream, command: Command) -> Result<(), io::Error> {
    match command {
        Command::Handshake => {
            let username = connection.read_mcstring()?;
            let random_number = random::<u128>();
            let random_hash = encode_string(md5::compute(random_number.to_le_bytes()).as_slice());

            info!("Handshake with {username} successful. Providing hash: {random_hash:?}");

            connection.write_u8(0x02)?;
            connection.write_mcstring(&MCString::try_from(random_hash).unwrap())?;
        },
        Command::Login => {
            info!("Initiating login");
            let protocol_version = connection.read_u32::<BE>()?;
            let username = connection.read_mcstring()?;
            let password = connection.read_mcstring()?;
            let map_seed = connection.read_i64::<BE>()?;
            let dimension = connection.read_i8()?;

            info!("Protocol Version: {protocol_version}");
            info!("Username: {username}");
            info!("Password: {password}");
            info!("Map Seed: {map_seed}");
            info!("Dimension: {dimension}");

            let login_packet = ServerLoginPacket {
                entity_id: 1,
                unknown1: MCString::default(),
                unknown2: MCString::default(),
                map_seed: 0,
                dimension: 0,
            };
            connection.write_u8(Command::Login as u8).unwrap();
            connection.write(&login_packet.to_bytes())?;

            info!("Responded to auth request");

            for i in -10..10 {
                for o in -10..10 {
                    let x = i * 16;
                    let z = o * 16;

                    connection.write_u8(Command::PreChunk as u8).unwrap();
                    connection.write_all(&PreChunk::new_load(x, z).to_bytes()).unwrap();

                    connection.write_u8(Command::MapChunk as u8)?;
                    connection.write_all(&MapChunk::new(x, z, BlockArray::new_superflat()).to_bytes())?;
                }
            }

            connection.write_u8(Command::SpawnPosition as u8)?;
            connection.write_u32::<BE>(0)?;
            connection.write_u32::<BE>(70)?;
            connection.write_u32::<BE>(0)?;

            let playerpos = PlayerPositionAndLook {
                position: PlayerPosition {
                    position_x: 1.0,
                    stance: 0.0,
                    position_y: 9.63,
                    position_z: 1.0,
                },
                look: PlayerLook {
                    yaw: 0.0,
                    pitch: 0.0,
                },
            };
            connection.write_u8(Command::PlayerPositionAndLook as u8)?;
            connection.write_all(&playerpos.to_bytes())?;
        },
        Command::ChatMessage => {
            info!("Chat Message Recieved: {}", connection.read_mcstring().unwrap());
        }
        Command::Player => {
            connection.read_u8()?;
        },
        Command::PlayerLook => {
            let _look = PlayerLook::from_bytes(&mut connection);
        }
        Command::PlayerPosition => {
            let _pos = PlayerPosition::from_bytes(&mut connection);
        }
        Command::PlayerPositionAndLook => {
            let _poslook = PlayerPositionAndLook::from_bytes(&mut connection);
        }
        Command::PlayerDigging => {
            let status = DiggingStatus::from_u8(connection.read_u8()?).unwrap();
            let pos_x = connection.read_i32::<BE>()?;
            let pos_y = connection.read_u8()?;
            let pos_z = connection.read_i32::<BE>()?;
            let face = connection.read_u8()?;
        }
        Command::ArmAnimation => {
            let eid = connection.read_i32::<BE>()?;
            let animate = connection.read_u8()? != 0;
        }
        Command::Disconnect => {
            let disconnect_string = connection.read_mcstring()?;
            info!("Disconnecting client: {disconnect_string}");
            connection.shutdown(std::net::Shutdown::Both)?;
        }
        Command::KeepAlive => {
            let _ = connection.write_u8(0x00);
            // TODO: Feed keepalive watchdog for client
        }
        Command::UpdateHealth => {
            connection.read_u8()?;
        }
        c => unimplemented!("This command ({c:?}) is probably `Server -> Client` only")
    }

    Ok(())
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[derive(FromPrimitive)]
enum DiggingStatus {
    StartedDigging = 0,
    Digging = 1,
    StoppedDigging = 2,
    BlockBroken = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[derive(FromPrimitive)]
enum Command {
    KeepAlive = 0x00,
    Login = 0x01,
    Handshake = 0x02,
    ChatMessage = 0x03,
    TimeUpdate = 0x04,
    PlayerInventory = 0x05,
    SpawnPosition = 0x06,
    UpdateHealth = 0x08,
    Respawn = 0x09,
    Player = 0x0A,
    PlayerPosition = 0x0B,
    PlayerLook = 0x0C,
    PlayerPositionAndLook = 0x0D,
    PlayerDigging = 0x0E,
    ArmAnimation = 0x12,
    PreChunk = 0x32,
    MapChunk = 0x33,
    Disconnect = 0xFF,
}

struct ServerLoginPacket {
    entity_id: i32,
    unknown1: MCString,
    unknown2: MCString,
    map_seed: i64,
    dimension: i8,
}

impl ToBytes for ServerLoginPacket {
    type Bytes = Vec<u8>;

    fn to_bytes(self) -> Self::Bytes {
        let mut out_buf = Vec::new();
        out_buf.write_i32::<BE>(self.entity_id).unwrap();
        out_buf.write_mcstring(&self.unknown1).unwrap();
        out_buf.write_mcstring(&self.unknown2).unwrap();
        out_buf.write_i64::<BE>(self.map_seed).unwrap();
        out_buf.write_i8(self.dimension).unwrap();

        out_buf
    }
}
