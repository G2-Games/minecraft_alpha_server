use std::collections::BTreeMap;

use crate::{
    blocks_items::BlockItem, chunk::MapChunk, position::{PlayerLook, PlayerPosition, PlayerPositionLook}
};

pub struct GameState {
    player_list: BTreeMap<String, PlayerState>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player_list: BTreeMap::new()
        }
    }

    pub fn player_list(&self) -> &BTreeMap<String, PlayerState> {
        &self.player_list
    }

    pub fn player_list_mut(&mut self) -> &mut BTreeMap<String, PlayerState> {
        &mut self.player_list
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    eid: i32,
    username: String,
    holding: BlockItem,
    position_look: PlayerPositionLook,
}

impl PlayerState {
    /// Create a new player when they join
    pub fn new(username: String, eid: i32,) -> Self {
        Self {
            eid,
            username,
            holding: BlockItem::Unknown,
            position_look: PlayerPositionLook::default(),
        }
    }

    pub fn new_invalid() -> Self {
        Self {
            eid: -1,
            username: String::new(),
            holding: BlockItem::Unknown,
            position_look: PlayerPositionLook::default(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.eid >= 0 && self.username != String::new() {
            true
        } else {
            false
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn look(&self) -> &PlayerLook {
        &self.position_look.look
    }

    pub fn position(&self) -> &PlayerPosition {
        &self.position_look.position
    }

    pub fn position_look(&self) -> &PlayerPositionLook {
        &self.position_look
    }

    pub fn holding(&self) -> &BlockItem {
        &self.holding
    }

    pub fn set_position(&mut self, position: PlayerPosition) {
        self.position_look.position = position
    }

    pub fn set_look(&mut self, look: PlayerLook) {
        self.position_look.look = look
    }

    pub fn set_holding(&mut self, holding: BlockItem) {
        self.holding = holding
    }
}

pub struct WorldState {
    chunks: Vec<MapChunk>,
}
