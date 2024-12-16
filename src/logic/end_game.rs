use crate::{
    basics::{
        events,
        storage::{self},
    },
    specific::{crashpoint, game_times::Timestamp, status::Status},
};

use multiversx_sc::imports::*;
const ANYONE_CAN_END_TIMESTAMP: Timestamp = 600; // 10 minutes

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
            caller == owner
                || curent_timestamp >= game_times.init_moment + ANYONE_CAN_END_TIMESTAMP,
            "Only Owner can end game for now"
        );

        self.compute_prizes();
    }

    #[endpoint]
    fn compute_prizes(&self) {
        let mut win_amount = BigUint::zero();

        let game_nonce = self.game_nonce().get();
        let crash_point = self.compute_crash_point();

        let mut contestants = self.contestants();
        for contestant in contestants.iter() {
            let bet = self.bet(&contestant).take();
            if bet.cash_out > crash_point {
                continue;
            }
            self.available_prize(&contestant)
                .update(|amount| *amount += &bet.amount * bet.cash_out);
            self.winner_announcement_event(&contestant, &(&bet.amount * bet.cash_out), game_nonce);
            win_amount += bet.amount * bet.cash_out;
        }
        contestants.clear();

        self.debt().update(|amount| *amount += win_amount);
        self.status().set(Status::Ended);
        self.ended_game_event(crash_point, game_nonce);
        self.game_nonce().update(|nonce| {
            *nonce += 1;
        });
    }
}
