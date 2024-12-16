use crate::{
    basics::{events, storage},
    specific::{bet::Bet, status::Status},
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait BettingModule: storage::StorageModule + events::EventsModule {
    #[payable("EGLD")]
    #[endpoint(submitBet)]
    fn submit_bet(&self, cash_out: u32) {
        require!(
            self.status().get() == Status::Ongoing,
            "game has not started yet"
        );
        let current_timestamp = self.blockchain().get_block_timestamp();
        let game_times = self.game_times().get();
        require!(
            game_times.init_moment + game_times.duration < current_timestamp,
            "bet submission has ended"
        );

        let caller = self.blockchain().get_caller();
        let payment = self.call_value().egld_value().clone_value();

        require!(self.bet(&caller).is_empty(), "cannot submit multiple bets");

        self.available_bet_amount().update(|available_bet_amount| {
            require!(
                &payment * cash_out / 100u32 <= available_bet_amount.clone(),
                "Betting unavailable"
            );
            *available_bet_amount -= &payment * cash_out;
        });

        self.user_bet_event(&caller, &payment, cash_out);
        self.bet(&caller).set(Bet {
            amount: payment,
            cash_out,
        });
        self.contestants().insert(caller);
    }
}
