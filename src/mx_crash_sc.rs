#![no_std]

use basics::{events, storage};
use logic::{awarding, claim, end_game, init_game, submit_bet};
use multiversx_sc::imports::*;
use specific::{crashpoint, status::Status};

mod basics;
mod logic;
pub mod mx_crash_sc_proxy;
mod specific;

#[multiversx_sc::contract]
pub trait MxCrashSc:
    init_game::InitGameModule
    + storage::StorageModule
    + submit_bet::BettingModule
    + crashpoint::CrashpointModule
    + end_game::EndGameModule
    + claim::ClaimModule
    + events::EventsModule
    + awarding::AwardingModule
{
    #[init]
    fn init(&self) {
        self.status().set(Status::Ended);
        self.game_nonce().set(0);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
