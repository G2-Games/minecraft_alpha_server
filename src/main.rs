mod utils;
mod byte_ops;
mod chunk;

use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}};

use base16ct::lower::encode_string;
use chunk::{BlockArray, MapChunk, PreChunk};
use log::{info, warn};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use byte_ops::ToBytes;
use utils::{MCString, ReadMCString, WriteMCString};
use rand::random;


fn main() {
    colog::default_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();

    for mut connection in listener.incoming().filter_map(|c| c.ok()) {
        info!("Connected to client @ {}", connection.peer_addr().unwrap());
        while let Some(cmd) = connection.read_u8().ok() {
            let command = Command::from_u8(cmd);
            info!("COMMAND: {command:?} (0x{cmd:02X?})");
            handle_command(&mut connection, command.unwrap()).unwrap();
        }
        warn!("Lost connection to client");
    }
}

fn handle_command(mut connection: &mut TcpStream, command: Command) -> Result<(), io::Error> {
    match command {
        Command::KeepAlive => todo!(),
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
                entity_id: 1200,
                unknown1: MCString::default(),
                unknown2: MCString::default(),
                map_seed: 1715505462032542147,
                dimension: 0,
            };
            connection.write_u8(Command::Login as u8).unwrap();
            connection.write(&login_packet.to_bytes())?;

            info!("Responded to auth request");

            for i in 0..7 {
                for o in 0..7 {
                    let x = (-4 + i) * 16;
                    let z = (-5 + o) * 16;

                    connection.write_u8(Command::PreChunk as u8).unwrap();
                    connection.write_all(&PreChunk::new_load(x, z).to_bytes()).unwrap();

                    connection.write_u8(Command::MapChunk as u8)?;
                    connection.write_all(&MapChunk::new(x, z, BlockArray::new_air()).to_bytes())?;
                }
            }

            connection.write_u8(Command::SpawnPosition as u8)?;
            connection.write_u32::<BE>(0)?;
            connection.write_u32::<BE>(70)?;
            connection.write_u32::<BE>(0)?;

            let playerpos = PlayerPositionAndLook {
                position_x: 0.0,
                stance: 0.0,
                position_y: 70.0,
                position_z: 0.0,
                yaw: 0.0,
                pitch: 0.0,
                on_ground: true,
            };
            connection.write_u8(Command::PlayerPositionAndLook as u8)?;
            connection.write_all(&playerpos.to_bytes())?;
        },
        Command::PlayerPositionAndLook => {
            let _poslook = PlayerPositionAndLook::from_bytes(&mut connection);
        }
        Command::PlayerPosition => {
            let _pos = PlayerPosition::from_bytes(&mut connection);
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
    PlayerPositionAndLook = 0x0D,
    PlayerPosition = 0x0B,
    PreChunk = 0x32,
    MapChunk = 0x33,
    Disconnect = 0xFF,
}

#[derive(Debug, Clone, Copy)]
struct CommandError {
    _id: u8,
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

#[derive(Debug, Clone, Copy)]
struct PlayerPositionAndLook {
    position_x: f64,
    stance: f64,
    position_y: f64,
    position_z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}

impl ToBytes for PlayerPositionAndLook {
    type Bytes = Vec<u8>;

    fn to_bytes(self) -> Self::Bytes {
        let mut out_buf = Vec::new();
        out_buf.write_f64::<BE>(self.position_x).unwrap();
        out_buf.write_f64::<BE>(self.stance).unwrap();
        out_buf.write_f64::<BE>(self.position_y).unwrap();
        out_buf.write_f64::<BE>(self.position_z).unwrap();
        out_buf.write_f32::<BE>(self.yaw).unwrap();
        out_buf.write_f32::<BE>(self.pitch).unwrap();
        out_buf.write_u8(self.on_ground as u8).unwrap();

        out_buf
    }
}

impl PlayerPositionAndLook {
    fn from_bytes<R: Read>(stream: &mut R) -> Self {
        let position_x = stream.read_f64::<BE>().unwrap();
        let position_y = stream.read_f64::<BE>().unwrap();
        let stance = stream.read_f64::<BE>().unwrap();
        let position_z = stream.read_f64::<BE>().unwrap();

        let yaw = stream.read_f32::<BE>().unwrap();
        let pitch = stream.read_f32::<BE>().unwrap();
        let on_ground = stream.read_u8().unwrap() != 0;

        Self {
            position_x,
            stance,
            position_y,
            position_z,
            yaw,
            pitch,
            on_ground
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PlayerPosition {
    position_x: f64,
    position_y: f64,
    stance: f64,
    position_z: f64,
    on_ground: bool,
}

impl ToBytes for PlayerPosition {
    type Bytes = Vec<u8>;

    fn to_bytes(self) -> Self::Bytes {
        let mut out_buf = Vec::new();
        out_buf.write_f64::<BE>(self.position_x).unwrap();
        out_buf.write_f64::<BE>(self.position_y).unwrap();
        out_buf.write_f64::<BE>(self.stance).unwrap();
        out_buf.write_f64::<BE>(self.position_z).unwrap();
        out_buf.write_u8(self.on_ground as u8).unwrap();

        out_buf
    }
}

impl PlayerPosition {
    fn from_bytes<R: Read>(stream: &mut R) -> Self {
        let position_x = stream.read_f64::<BE>().unwrap();
        let position_y = stream.read_f64::<BE>().unwrap();
        let stance = stream.read_f64::<BE>().unwrap();
        let position_z = stream.read_f64::<BE>().unwrap();

        let on_ground = stream.read_u8().unwrap() != 0;

        Self {
            position_x,
            stance,
            position_y,
            position_z,
            on_ground
        }
    }
}
