mod mcstring;
mod byte_ops;
mod chunk;
mod position;
mod state;
mod player;
mod blocks_items;
mod entity_id;
mod packets;

use std::{io::{self, Write}, net::{TcpListener, TcpStream}, process::exit, sync::{Arc, RwLock}, thread};

use base16ct::lower::encode_string;
use chunk::{BlockArray, MapChunk, PreChunk};
use entity_id::ENTITY_ID;
use log::{debug, error, info};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use byte_ops::ToBytes;
use packets::{packet15_place::Packet15Place, packet1_login, Packet};
use player::DiggingStatus;
use position::{PlayerLook, PlayerPosition, PlayerPositionLook};
use state::{GameState, PlayerState};
use mcstring::{MCString, ReadMCString, WriteMCString};
use rand::random;

fn main() {
    colog::default_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    info!("Starting Minecraft server version Beta 1.1_02");
    let game_state: Arc<RwLock<GameState>> = Arc::new(RwLock::new(GameState::new()));

    let listener = match TcpListener::bind("0.0.0.0:25565") {
        Ok(l) => l,
        Err(e) => {
            error!("Starting server failed: {e}");
            exit(1)
        },
    };
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

        if player_state.is_valid()
            && (game_state.read().unwrap().player_list().get(player_state.username()).is_some_and(|p| *p != player_state)
            || game_state.read().unwrap().player_list().get(player_state.username()).is_none())
        {
             game_state.write()
                 .unwrap()
                 .player_list_mut()
                 .insert(player_state.username().clone(), player_state.clone());
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
            let login_info = packet1_login::Packet1Login::read_from(&mut connection)?;

            // Return a successful login packet to the client
            let eid = ENTITY_ID.get();
            let login_packet = packet1_login::Packet1Login::new(eid, 0, 0);
            connection.write_u8(Command::Login as u8)?;
            login_packet.write_into(&mut connection)?;

            info!("{} [{}] logged in with entity id {}", login_info.username, connection.peer_addr().unwrap(), eid);

            *player_state = PlayerState::new(login_info.username.to_string(), eid);

            // Send "chunks" to the player. This simulates a flat-world of 20 by 20 chunks.
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
            let _block_id = connection.read_i16::<BE>()?;
        }
        Command::PlayerDigging => {
            let _status = DiggingStatus::from_u8(connection.read_u8()?).unwrap();
            let _pos_x = connection.read_i32::<BE>()?;
            let _pos_y = connection.read_u8()?;
            let _pos_z = connection.read_i32::<BE>()?;
            let _face = connection.read_u8()?;
        }
        Command::PlayerBlockPlacement => {
            let status = Packet15Place::read_from(&mut connection)?;
            dbg!(status);
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
            info!("Keepalive!");
        }
        Command::UpdateHealth => {
            connection.read_u8()?;
        }
        c => unimplemented!("This command ({c:?}) is probably `Server -> Client` only; thus it is unimplemented for the other way around!")
    }

    connection.write_u8(Command::KeepAlive as u8)?;

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
