#![no_std]

use basics::{
    constants::{DEFAULT_INSTANT_CRASH_CHANCE, ONE_MINUTE, TEN_MINUTES},
    events, storage, views,
};
use logic::{awarding, claim, end_game, init_game, submit_bet};
use multiversx_sc::imports::*;
use specific::{
    crashpoint,
    game_times::{GameTimes, Timestamp},
    status::Status,
};

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
    + multiversx_sc_modules::only_admin::OnlyAdminModule
    + multiversx_sc_modules::pause::PauseModule
{
    #[init]
    fn init(&self) {
        self.status().set(Status::Ended);
        self.game_nonce().set(0);
        self.game_times().set(GameTimes {
            duration: 0,
            init_moment: self.blockchain().get_block_timestamp(),
        });
        self.game_duration().set(ONE_MINUTE);
        self.instant_crash_chance()
            .set(DEFAULT_INSTANT_CRASH_CHANCE);
        self.set_paused(false);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[only_admin]
    #[payable("EGLD")]
    #[endpoint(deposit)]
    fn deposit(&self) {
        self.require_not_paused();
    }

    #[only_admin]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        self.require_not_paused();

        let caller = self.blockchain().get_caller();
        let sc_address = self.blockchain().get_sc_address();

        let balance = self.blockchain().get_balance(&sc_address);
        let debt = self.debt().get();

        self.send().direct_egld(&caller, &(balance - debt));
    }

    #[endpoint(givePermission)]
    fn give_permission(&self, permitted_address: ManagedAddress) {
        self.require_not_paused();
        let caller = self.blockchain().get_caller();
        self.user_permission(&caller).set(permitted_address);
    }

    #[endpoint(revokePermission)]
    fn revoke_permission(&self) {
        self.require_not_paused();
        let caller = self.blockchain().get_caller();
        self.user_permission(&caller).clear();
    }

    #[only_owner]
    #[endpoint(setDuration)]
    fn set_duration(&self, duration: Timestamp) {
        require!(
            duration <= TEN_MINUTES,
            "Duration cannot be greater than 10 min"
        );
        self.game_duration().set(duration);
    }

    #[only_owner]
    #[endpoint(setInstantCrashChance)]
    fn set_instant_crash_chance(&self, chance: u64) {
        self.instant_crash_chance().set(chance);
    }
}
