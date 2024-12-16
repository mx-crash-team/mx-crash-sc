#![no_std]

use basics::{events, storage, views};
use logic::{awarding, claim, end_game, init_game, submit_bet};
use multiversx_sc::imports::*;
use specific::{crashpoint, game_times::GameTimes, status::Status};

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
    + views::ViewsModule
{
    #[init]
    fn init(&self) {
        self.status().set(Status::Ended);
        self.game_nonce().set(0);
        self.game_times().set(GameTimes {
            duration: 0,
            init_moment: self.blockchain().get_block_timestamp(),
        });
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint]
    fn deposit(&self) {}

    #[only_owner]
    #[endpoint]
    fn withdraw(&self) {
        require!(
            self.status().get() == Status::Ended,
            "a game is currently ongoing"
        );

        let caller = self.blockchain().get_caller();
        let sc_address = self.blockchain().get_sc_address();

        let balance = self.blockchain().get_balance(&sc_address);
        let debt = self.debt().get();

        self.send().direct_egld(&caller, &(balance - debt));
    }
}
