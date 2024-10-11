mod utils;
mod byte_ops;
mod chunk;
mod position;
mod state;
mod player;
mod blocks_items;

use std::{io::{self, Write}, net::{TcpListener, TcpStream}, sync::{atomic::{self, AtomicI32}, Arc, RwLock}, thread};

use base16ct::lower::encode_string;
use chunk::{BlockArray, MapChunk, PreChunk};
use log::{error, info, warn};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use byte_ops::ToBytes;
use player::{DiggingStatus, PlayerBlockPlacement};
use position::{PlayerLook, PlayerPosition, PlayerPositionLook};
use state::{GameState, PlayerState};
use utils::{MCString, ReadMCString, WriteMCString};
use rand::random;

/// The current Entity ID. Incremented by one every time there is a new entity.
///
/// This value should rarely be accessed directly, and definitely never updated.
static ENTITY_ID: AtomicI32 = AtomicI32::new(0);

/// Get an Entity ID and increment the global value by 1.
#[inline]
fn get_eid() -> i32 {
    let eid = ENTITY_ID.load(atomic::Ordering::Relaxed);
    ENTITY_ID.store(eid + 1, atomic::Ordering::Relaxed);

    eid
}

fn main() {
    colog::default_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    info!("Setting up game state");
    let game_state: Arc<RwLock<GameState>> = Arc::new(RwLock::new(GameState::new()));

    let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
    info!("Server started and listening on {}", listener.local_addr().unwrap());

    for mut connection in listener.incoming().filter_map(|c| c.ok()) {
        info!("Player joined from {}", connection.peer_addr().unwrap());
        let mut game_state = Arc::clone(&game_state);
        thread::spawn(move || {
            player_loop(
                &mut connection,
                &mut game_state,
            ).unwrap();

            info!("Connection dropped for {}", connection.peer_addr().unwrap());
        });
    }
}

fn player_loop(
    mut connection: &mut TcpStream,
    game_state: &mut Arc<RwLock<GameState>>,
) -> Result<(), io::Error> {
    let mut player_state = PlayerState::new_invalid();
    loop {
        if let Ok(cmd) = connection.read_u8() {
            let command = Command::from_u8(cmd);
            if command.is_none() {
                error!("COMMAND: {command:?} (0x{cmd:02X?})");
                panic!("This command isn't implemented yet");
            }

            handle_command(
                &mut connection,
                command.unwrap(),
                &mut player_state,
            ).unwrap();
        } else {
            break;
        }

        if player_state.is_valid() {
            if game_state.read().unwrap().player_list().get(player_state.username()).is_some_and(|p| *p != player_state)
                || game_state.read().unwrap().player_list().get(player_state.username()).is_none()
            {
                game_state.write()
                    .unwrap()
                    .player_list_mut()
                    .insert(player_state.username().clone(), player_state.clone());
            }
        }

    }

    Ok(())
}

fn handle_command(
    mut connection: &mut TcpStream,
    command: Command,
    player_state: &mut PlayerState,
) -> Result<(), io::Error> {
    match command {
        Command::Handshake => {
            let username = connection.read_mcstring()?;
            let random_number = random::<u128>();
            let random_hash = encode_string(md5::compute(random_number.to_le_bytes()).as_slice());

            connection.write_u8(Command::Handshake as u8)?;
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

            // Return a successful login packet to the client
            let eid = get_eid();
            let login_packet = ServerLoginPacket::new(eid, 0, 0);
            connection.write_u8(Command::Login as u8).unwrap();
            connection.write_all(&login_packet.to_bytes())?;

            info!("{username} logged in. Protocol version {protocol_version}");
            *player_state = PlayerState::new(username.to_string(), eid);

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
            connection.write_u32::<BE>(0)?;
            connection.write_u32::<BE>(0)?;

            let playerpos = PlayerPositionLook {
                position: PlayerPosition {
                    position_x: 0.5,
                    stance: 0.0,
                    position_y: 9.63,
                    position_z: 0.5,
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
            let message = connection.read_mcstring().unwrap();
            info!("Chat Message Recieved: {message}");
        }
        Command::Player => {
            connection.read_u8()?;
        },
        Command::PlayerLook => {
            let look = PlayerLook::from_bytes(&mut connection);
            player_state.set_look(look);
        }
        Command::PlayerPosition => {
            let pos = PlayerPosition::from_bytes(&mut connection);
            player_state.set_position(pos);
        }
        Command::PlayerPositionAndLook => {
            let poslook = PlayerPositionLook::from_bytes(&mut connection);
            player_state.set_look(poslook.look);
            player_state.set_position(poslook.position);
        }
        Command::HoldingChange => {
            let _unused = connection.read_i32::<BE>()?;
            let block_id = connection.read_i16::<BE>()?;
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
        }
        Command::Animation => {
            let _eid = connection.read_i32::<BE>()?;
            let _animate = connection.read_u8()?;
        }
        Command::Disconnect => {
            let disconnect_string = connection.read_mcstring()?;
            info!("Disconnecting client. Reason: {disconnect_string}");
            connection.shutdown(std::net::Shutdown::Both)?;
        }
        Command::KeepAlive => {
            connection.write_u8(Command::KeepAlive as u8)?;
        }
        Command::UpdateHealth => {
            connection.read_u8()?;
        }
        c => unimplemented!("This command ({c:?}) is probably `Server -> Client` only; thus it is unimplemented for the other way around!")
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
    HoldingChange = 0x10,
    AddToInventory = 0x11,
    Animation = 0x12,
    NamedEntitySpawn = 0x14,
    PickupSpawn = 0x15,
    CollectItem = 0x16,
    AddObject = 0x17,
    MobSpawn = 0x18,
    EntityVelocity = 0x1C,
    DestroyEntity = 0x1D,
    Entity = 0x1E,
    EntityRelativeMove = 0x1F,
    EntityLook = 0x20,
    EntityLookAndRelativeMove = 0x21,
    EntityTeleport = 0x22,
    AttachEntity = 0x27,
    PreChunk = 0x32,
    MapChunk = 0x33,
    BlockChange = 0x35,
    ComplexEntities = 0x3B,
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
