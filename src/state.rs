use crate::{
    chunk::MapChunk, position::{PlayerLook, PlayerPosition},
};

pub struct GameState {

}

pub struct PlayerState {
    eid: i32,
    username: String,
    position: PlayerPosition,
    look: PlayerLook,
}

impl PlayerState {
    /// Create a new player when they join
    pub fn new(username: String, eid: i32,) -> Self {
        Self {
            eid,
            username,
            position: PlayerPosition::default(),
            look: PlayerLook::default(),
        }
    }
}

pub struct WorldState {
    chunks: Vec<MapChunk>,
}
