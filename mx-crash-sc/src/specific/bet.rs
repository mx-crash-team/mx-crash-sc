use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, Debug)]
pub struct Bet<M>
where
    M: ManagedTypeApi,
{
    pub amount: BigUint<M>,
    pub cash_out: u32,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, Debug)]
pub struct ContestantDetails<M>
where
    M: ManagedTypeApi,
{
    pub address: ManagedAddress<M>,
    pub amount: BigUint<M>,
    pub cash_out: u32,
}
