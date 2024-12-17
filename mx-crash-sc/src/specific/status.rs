use multiversx_sc::derive_imports::*;

use super::game_times::Timestamp;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
pub enum Status {
    Ongoing,
    Ended,
    Awarding,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
pub struct GameDetails {
    pub status: Status,
    pub nonce: u32,
    pub duration: Timestamp,
    pub init_moment: Timestamp,
    pub current_timestamp: Timestamp,
}
