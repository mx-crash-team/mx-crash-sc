use crate::{
    basics::{events, storage},
    specific::{bet::Bet, status::Status},
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait BettingModule: storage::StorageModule + events::EventsModule {
    #[payable("EGLD")]
    #[endpoint(submitBet)]
    fn submit_bet(&self, cash_out: u32, optional_contestant: OptionalValue<ManagedAddress>) {
        require!(
            self.status().get() == Status::Ongoing,
            "game has not started yet"
        );
        let current_timestamp = self.blockchain().get_block_timestamp();
        let game_times = self.game_times().get();
        require!(
            game_times.init_moment + game_times.duration > current_timestamp,
            "bet submission has ended"
        );

        let caller = self.blockchain().get_caller();

        let contestant: ManagedAddress = if optional_contestant.is_none() {
            caller
        } else {
            let user = optional_contestant.into_option().unwrap();
            require!(
                self.user_permission(&user).get() == caller,
                "Not allowed to bet for user"
            );
            user
        };
        let payment = self.call_value().egld().clone_value();

        require!(
            self.bet(&contestant).is_empty(),
            "cannot submit multiple bets"
        );

        let aimed_win_amount = &payment * cash_out / 100u32;
        self.available_bet_amount().update(|available_bet_amount| {
            require!(
                aimed_win_amount <= available_bet_amount.clone(),
                "Betting unavailable"
            );
            *available_bet_amount -= aimed_win_amount;
        });

        self.user_bet_event(&contestant, &payment, cash_out);
        self.bet(&contestant).set(Bet {
            amount: payment,
            cash_out,
        });
        self.contestants().insert(contestant);
    }
}
