use crate::storage;
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait UserOperationsModule: storage::StorageModule {
    #[endpoint]
    #[payable("EGLD")]
    fn deposit(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().egld_value().clone_value();
        self.deposited_funds(&caller).update(|amount| {
            *amount += &payment;
        });
    }

    #[endpoint]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let amount = self.deposited_funds(&caller).take();
        require!(amount > 0, "nothing to withdraw");
        self.send().direct_egld(&caller, &amount);
    }
}
