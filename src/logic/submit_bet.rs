use crate::{
    basics::{events, storage},
    specific::bet::Bet,
};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait BettingModule: storage::StorageModule + events::EventsModule {
    #[payable("EGLD")]
    #[endpoint(submitBet)]
    fn submit_bet(&self, cash_out: u32) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().egld_value().clone_value();

        require!(self.bet(&caller).is_empty(), "cannot submit multiple bets");

        self.available_bet_amount().update(|available_bet_amount| {
            require!(
                &payment * cash_out <= available_bet_amount.clone(),
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
