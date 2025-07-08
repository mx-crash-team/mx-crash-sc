use multiversx_sc::imports::*;

use crate::specific::{
    bet::Bet,
    game_times::{GameTimes, Timestamp},
    status::Status,
};

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("gameTimes")]
    fn game_times(&self) -> SingleValueMapper<GameTimes>;

    #[storage_mapper("availableBetAmount")]
    fn available_bet_amount(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("debt")]
    fn debt(&self) -> SingleValueMapper<BigUint>;

    #[view]
    #[storage_mapper("status")]
    fn status(&self) -> SingleValueMapper<Status>;

    #[view]
    #[storage_mapper("gameDuration")]
    fn game_duration(&self) -> SingleValueMapper<Timestamp>;

    #[storage_mapper("instantCrashChance")]
    fn instant_crash_chance(&self) -> SingleValueMapper<u64>;

    #[view]
    #[storage_mapper("gameNonce")]
    fn game_nonce(&self) -> SingleValueMapper<u32>;

    #[view]
    #[storage_mapper("crashPoint")]
    fn crash_point(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("bet")]
    fn bet(&self, address: &ManagedAddress) -> SingleValueMapper<Bet<Self::Api>>;

    #[view]
    #[storage_mapper("contestants")]
    fn contestants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("availablePrize")]
    fn available_prize(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("userPermission")]
    fn user_permission(&self, address: &ManagedAddress) -> SingleValueMapper<ManagedAddress>;
}
