#![allow(non_snake_case)]

pub mod config;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";
const MX_CRASH_SC_DIR: &str = "mx_crash_sh";
const KEYSTORE_DIR_PATH: &str = "../../keystores/keystore.json";

pub async fn mx_crash_sc_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new(Config::chain_simulator_config()).await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "upgrade" => interact.upgrade().await,
        "withdraw" => interact.withdraw().await,
        "givePermission" => interact.give_permission().await,
        "revokePermission" => interact.revoke_permission().await,
        "newGame" => interact.new_game().await,
        "status" => interact.status().await,
        "game_nonce" => interact.game_nonce().await,
        // "crash_point" => interact.crash_point().await,
        "contestants" => interact.contestants().await,
        "available_prize" => interact.available_prize().await,
        "endGame" => interact.end_game().await,
        // "claim" => interact.claim().await,
        "computePrizes" => interact.compute_prizes().await,
        "getGameDetails" => interact.get_game_details().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

pub struct ContractInteract {
    pub interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State,
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace(MX_CRASH_SC_DIR);
        let wallet_address = interactor
            .register_wallet(
                Wallet::from_keystore_secret(KEYSTORE_DIR_PATH, InsertPassword::StandardInput)
                    .unwrap(),
            )
            .await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/mx-crash-sc.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = Bech32Address::from(&new_address);
        self.state.set_address(new_address_bech32.clone());

        println!("new address: {new_address_bech32}");
    }

    pub async fn upgrade(&mut self) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .upgrade()
            .code(&self.contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn deposit(&mut self, egld_amount: BigUint<StaticApi>) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .deposit()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn withdraw(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .withdraw()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn give_permission(&mut self) {
        let permitted_zero_address = Bech32Address::zero("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .give_permission(permitted_zero_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn revoke_permission(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .revoke_permission()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn new_game(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .new_game()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn status(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MxCrashScProxy)
            .status()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn game_nonce(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MxCrashScProxy)
            .game_nonce()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn crash_point(&mut self) -> u32 {
        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MxCrashScProxy)
            .crash_point()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn contestants(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MxCrashScProxy)
            .contestants()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn available_prize(&mut self) {
        let zero_address = Bech32Address::zero("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MxCrashScProxy)
            .available_prize(zero_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn submit_bet(
        &mut self,
        caller: &Address,
        cash_out: u32,
        egld_amount: BigUint<StaticApi>,
    ) {
        let optional_contestant: OptionalValue<ManagedAddress<StaticApi>> = OptionalValue::None;

        let response = self
            .interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .submit_bet(cash_out, optional_contestant)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn end_game(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .end_game()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn claim(&mut self, caller: Address) {
        let response = self
            .interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .claim()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_duration(&mut self, duration: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .set_duration(duration)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_admin(&mut self, new_admin_address: Address) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .add_admin(new_admin_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_prizes(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MxCrashScProxy)
            .compute_prizes()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_game_details(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MxCrashScProxy)
            .get_game_details()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }
}
