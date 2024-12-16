use crate::{
    basics::{
        constants::TEN_MINUTES,
        events,
        storage::{self},
    },
    specific::{crashpoint, status::Status},
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait EndGameModule:
    storage::StorageModule + crashpoint::CrashpointModule + events::EventsModule
{
    #[endpoint(endGame)]
    fn end_game(&self) {
        require!(
            self.status().get() == Status::Ongoing,
            "game was already ended"
        );
        let caller = self.blockchain().get_caller();
        let owner = self.blockchain().get_owner_address();

        let curent_timestamp = self.blockchain().get_block_timestamp();
        let game_times = self.game_times().get();

        require!(
            caller == owner || curent_timestamp >= game_times.init_moment + TEN_MINUTES,
            "Only Owner can end game for now"
        );

        let game_nonce = self.game_nonce().get();
        let crash_point = if game_nonce % 33 == 1 {
            game_nonce
        } else {
            self.compute_crash_point()
        };
        self.crash_point().set(&crash_point);

        self.status().set(Status::Awarding);
        self.ended_game_event(crash_point, game_nonce);
    }
}
