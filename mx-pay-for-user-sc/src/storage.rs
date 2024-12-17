use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("crashScAddress")]
    fn crash_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("depositedFunds")]
    fn deposited_funds(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
