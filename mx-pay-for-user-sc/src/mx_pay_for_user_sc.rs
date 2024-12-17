#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
mod bot_operations;
mod mx_crash_sc_proxy;
mod storage;
mod user_operations;

#[multiversx_sc::contract]
pub trait MxPayForUserSc:
    user_operations::UserOperationsModule + bot_operations::BotOperationsModule + storage::StorageModule
{
    #[init]
    fn init(&self, crash_sc_address: ManagedAddress) {
        self.crash_sc_address().set(crash_sc_address);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
