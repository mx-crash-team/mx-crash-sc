use crate::{
    basics::{
        events,
        storage::{self},
    },
    specific::{crashpoint, game_times::GameTimes, status::Status},
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait InitGameModule:
    storage::StorageModule
    + crashpoint::CrashpointModule
    + events::EventsModule
    + multiversx_sc_modules::pause::PauseModule
{
    #[endpoint(newGame)]
    fn new_game(&self) {
        self.require_not_paused();

        require!(
            self.status().get() == Status::Ended,
            "Another game is currently ongoing"
        );
        let init_moment = self.blockchain().get_block_timestamp();
        let duration = self.game_duration().get();
        self.game_times().set(GameTimes {
            duration,
            init_moment,
        });
        let sc_address = self.blockchain().get_sc_address();
        let balance = self.blockchain().get_balance(&sc_address);
        let debt = self.debt().get();

        self.available_bet_amount().set(balance - debt);
        self.status().set(Status::Ongoing);
        self.started_game_event(self.game_nonce().get());
    }
}
