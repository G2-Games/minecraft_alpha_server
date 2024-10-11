mod utils;
mod byte_ops;
mod chunk;
mod position;
mod state;
mod player;

use std::{io::{self, Write}, net::{TcpListener, TcpStream}, sync::RwLock};

use base16ct::lower::encode_string;
use chunk::{BlockArray, MapChunk, PreChunk};
use log::{info, warn};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use byte_ops::ToBytes;
use player::{DiggingStatus, PlayerBlockPlacement};
use position::{PlayerLook, PlayerPosition, PlayerPositionAndLook};
use state::PlayerState;
use utils::{MCString, ReadMCString, WriteMCString};
use rand::random;

/// List of players.
const PLAYER_LIST: RwLock<Vec<PlayerState>> = RwLock::new(Vec::new());

/// The current Entity ID. Incremented by one every time there is a new entity.
const ENTITY_ID: RwLock<i32> = RwLock::new(0);

fn get_eid() -> i32 {
    let eid = ENTITY_ID.read().unwrap().clone();
    *ENTITY_ID.write().unwrap() += 1;

    eid
}

fn main() {
    colog::default_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
    info!("Server started and listening on {}", listener.local_addr().unwrap());

    for mut connection in listener.incoming().filter_map(|c| c.ok()) {
        info!("Player joined from {}", connection.peer_addr().unwrap());
        while let Some(cmd) = connection.read_u8().ok() {
            let command = Command::from_u8(cmd);
            if command.is_none() {
                info!("COMMAND: {command:?} (0x{cmd:02X?})");
                panic!("This command isn't implemented yet");
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

            connection.write_u8(0x02)?;
            connection.write_mcstring(&MCString::try_from(random_hash).unwrap())?;

            info!("Handshake with {username} successful");
        },
        Command::Login => {
            let protocol_version = connection.read_u32::<BE>()?;
            let username = connection.read_mcstring()?;
            // These are mostly useless
            let _password = connection.read_mcstring()?;
            let _map_seed = connection.read_i64::<BE>()?;
            let _dimension = connection.read_i8()?;

            let eid = get_eid();
            let login_packet = ServerLoginPacket {
                entity_id: eid,
                unknown1: MCString::default(),
                unknown2: MCString::default(),
                map_seed: 0,
                dimension: 0,
            };
            connection.write_u8(Command::Login as u8).unwrap();
            connection.write(&login_packet.to_bytes())?;

            PLAYER_LIST.write().unwrap().push(PlayerState::new(username.to_string(), eid));

            info!("{username} logged in. Protocol version {protocol_version}");

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
            let _status = DiggingStatus::from_u8(connection.read_u8()?).unwrap();
            let _pos_x = connection.read_i32::<BE>()?;
            let _pos_y = connection.read_u8()?;
            let _pos_z = connection.read_i32::<BE>()?;
            let _face = connection.read_u8()?;
        }
        Command::PlayerBlockPlacement => {
            let _status = PlayerBlockPlacement::from_bytes(&mut connection);
            dbg!(_status);
        }
        Command::ArmAnimation => {
            let _eid = connection.read_i32::<BE>()?;
            let _animate = connection.read_u8()? != 0;
            dbg!(_animate);
        }
        Command::Disconnect => {
            let disconnect_string = connection.read_mcstring()?;
            info!("Disconnecting client. Reason: {disconnect_string}");
            connection.shutdown(std::net::Shutdown::Both)?;
        }
        Command::KeepAlive => {
            let _ = connection.write_u8(0x00);
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
    PlayerBlockPlacement = 0x0F,
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

impl ServerLoginPacket {
    pub fn new(entity_id: i32, map_seed: i64, dimension: i8) -> Self {
        Self {
            entity_id,
            unknown1: MCString::default(),
            unknown2: MCString::default(),
            map_seed,
            dimension,
        }
    }
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
