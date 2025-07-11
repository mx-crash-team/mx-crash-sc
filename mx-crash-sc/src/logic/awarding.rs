use crate::{
    basics::{
        constants::MAX_CONTESTANTS_CHECKED,
        events,
        storage::{self},
    },
    specific::{crashpoint, status::Status},
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait AwardingModule:
    storage::StorageModule
    + crashpoint::CrashpointModule
    + events::EventsModule
    + multiversx_sc_modules::pause::PauseModule
{
    #[endpoint(computePrizes)]
    fn compute_prizes(&self) {
        self.require_not_paused();
        require!(
            self.status().get() == Status::Awarding,
            "game was already ended"
        );
        let mut win_amount = BigUint::zero();

        let game_nonce = self.game_nonce().get();
        let crash_point = self.crash_point().get();

        let mut contestants = self.contestants();
        let mut checked_contestants: ManagedVec<ManagedAddress> = ManagedVec::new();
        let mut addresses_checked = 0;
        for contestant in contestants.iter() {
            addresses_checked += 1;
            checked_contestants.push(contestant.clone());

            if addresses_checked > MAX_CONTESTANTS_CHECKED {
                break;
            }

            let bet = self.bet(&contestant).take();
            if bet.cash_out > crash_point {
                continue;
            }
            let prize_amount = &bet.amount * bet.cash_out / 100u32;
            self.available_prize(&contestant)
                .update(|amount| *amount += &prize_amount);
            self.winner_announcement_event(&contestant, &prize_amount, bet.cash_out, game_nonce);
            win_amount += prize_amount;
        }
        for contestant in checked_contestants.iter() {
            contestants.swap_remove(&contestant);
        }

        self.debt().update(|amount| *amount += win_amount);
        if contestants.is_empty() {
            self.status().set(Status::Ended);
            self.game_nonce().update(|nonce| {
                *nonce += 1;
            });
            self.ended_awarding_event(game_nonce);
        }
    }
}
