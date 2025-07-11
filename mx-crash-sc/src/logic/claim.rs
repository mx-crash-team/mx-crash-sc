use crate::basics::storage;

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ClaimModule: storage::StorageModule + multiversx_sc_modules::pause::PauseModule {
    #[endpoint(claim)]
    fn claim(&self) {
        self.require_not_paused();
        let caller = self.blockchain().get_caller();

        require!(
            !self.available_prize(&caller).is_empty(),
            "Nothing to claim"
        );

        let prize = self.available_prize(&caller).take();
        self.debt().update(|amount| *amount -= &prize);
        self.send().direct_egld(&caller, &prize);
    }
}
