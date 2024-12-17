use crate::{mx_crash_sc_proxy, storage};
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait BotOperationsModule: storage::StorageModule {
    #[endpoint]
    fn submit_bet_for_user(&self, bet: BigUint, cash_out: u32, user: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        require!(
            !self.deposited_funds(&caller).is_empty(),
            "unavailable funds"
        );
        self.deposited_funds(&caller).update(|amount| {
            require!(&bet <= amount, "unavailable funds");
            *amount -= &bet;
        });

        self.tx()
            .to(self.crash_sc_address().get())
            .typed(mx_crash_sc_proxy::MxCrashScProxy)
            .submit_bet(cash_out, OptionalValue::Some(user))
            .egld(bet)
            .sync_call()
    }
}
