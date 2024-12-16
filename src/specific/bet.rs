use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Bet<M>
where
    M: ManagedTypeApi,
{
    pub amount: BigUint<M>,
    pub cash_out: u32,
}
