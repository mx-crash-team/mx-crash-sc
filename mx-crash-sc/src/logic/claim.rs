use crate::basics::storage;

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ClaimModule: storage::StorageModule {
    #[endpoint(claim)]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            !self.available_prize(&caller).is_empty(),
            "unavailable claim"
        );

        let prize = self.available_prize(&caller).take();
        self.debt().update(|amount| *amount -= &prize);
        self.send().direct_egld(&caller, &prize);
    }
}
