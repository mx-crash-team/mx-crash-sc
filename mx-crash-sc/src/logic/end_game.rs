use crate::{
    basics::{
        events,
        storage::{self},
    },
    specific::{crashpoint, status::Status},
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait EndGameModule:
    storage::StorageModule
    + crashpoint::CrashpointModule
    + events::EventsModule
    + multiversx_sc_modules::only_admin::OnlyAdminModule
{
    #[only_admin]
    #[endpoint(endGame)]
    fn end_game(&self) {
        require!(
            self.status().get() == Status::Ongoing,
            "Game was already ended"
        );

        let curent_timestamp = self.blockchain().get_block_timestamp();
        let game_times = self.game_times().get();

        require!(
            curent_timestamp >= game_times.init_moment + self.game_duration().get(),
            "The needed time frame has not been reached"
        );

        let game_nonce = self.game_nonce().get();
        let crash_point = self.compute_crash_point();
        self.crash_point().set(crash_point);
        sc_print!("crash point: {}", crash_point);

        self.status().set(Status::Awarding);
        self.ended_game_event(crash_point, game_nonce);
    }
}
