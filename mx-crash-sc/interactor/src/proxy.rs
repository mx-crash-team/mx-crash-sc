// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct MxCrashScProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for MxCrashScProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = MxCrashScProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        MxCrashScProxyMethods { wrapped_tx: tx }
    }
}

pub struct MxCrashScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> MxCrashScProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> MxCrashScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> MxCrashScProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn deposit(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("deposit")
            .original_result()
    }

    pub fn withdraw(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdraw")
            .original_result()
    }

    pub fn give_permission<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        permitted_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("givePermission")
            .argument(&permitted_address)
            .original_result()
    }

    pub fn revoke_permission(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("revokePermission")
            .original_result()
    }

    pub fn new_game<
        Arg0: ProxyArg<u64>,
    >(
        self,
        duration: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("newGame")
            .argument(&duration)
            .original_result()
    }

    pub fn status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Status> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("status")
            .original_result()
    }

    pub fn game_nonce(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("game_nonce")
            .original_result()
    }

    pub fn crash_point(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("crash_point")
            .original_result()
    }

    pub fn contestants(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("contestants")
            .original_result()
    }

    pub fn available_prize<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("available_prize")
            .argument(&address)
            .original_result()
    }

    pub fn submit_bet<
        Arg0: ProxyArg<u32>,
        Arg1: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        cash_out: Arg0,
        optional_contestant: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("submitBet")
            .argument(&cash_out)
            .argument(&optional_contestant)
            .original_result()
    }

    pub fn end_game(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("endGame")
            .original_result()
    }

    pub fn claim(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claim")
            .original_result()
    }

    pub fn compute_prizes(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("computePrizes")
            .original_result()
    }

    pub fn get_game_details(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, GameDetails> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getGameDetails")
            .original_result()
    }

    pub fn contestant_details(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, ContestantDetails<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getContestantDetails")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
pub enum Status {
    Ongoing,
    Ended,
    Awarding,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
pub struct GameDetails {
    pub status: Status,
    pub nonce: u32,
    pub duration: u64,
    pub init_moment: u64,
    pub current_timestamp: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, Debug)]
pub struct ContestantDetails<Api>
where
    Api: ManagedTypeApi,
{
    pub address: ManagedAddress<Api>,
    pub amount: BigUint<Api>,
    pub cash_out: u32,
}
