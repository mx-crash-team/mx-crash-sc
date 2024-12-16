use crate::{
    basics::{
        events,
        storage::{self},
    },
    specific::{
        crashpoint,
        game_times::{GameTimes, Timestamp},
        status::Status,
    },
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait InitGameModule:
    storage::StorageModule + crashpoint::CrashpointModule + events::EventsModule
{
    #[only_owner]
    #[endpoint(newGame)]
    fn new_game(&self, duration: Timestamp) {
        require!(
            self.status().get() == Status::Ended,
            "another game is curently ongoing"
        );
        let init_moment = self.blockchain().get_block_timestamp();
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
