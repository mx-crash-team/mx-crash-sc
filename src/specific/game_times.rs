use multiversx_sc::derive_imports::*;

pub type Timestamp = u64;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct GameTimes {
    pub duration: Timestamp,
    pub init_moment: Timestamp,
}
