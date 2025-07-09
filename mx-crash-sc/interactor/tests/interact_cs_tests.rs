use multiversx_sc_snippets::imports::*;
use rust_interact::config::{ChainType, Config};
use rust_interact::ContractInteract;
pub const ONE_EGLD: u128 = 100_000_000_000_000_000u128;

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn devnet_deploy() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.deploy().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn devnet_new_game() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.new_game().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn devnet_end_game() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.end_game().await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn devnet_set_duration() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    interactor.set_duration(210).await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn devnet_get_game_details() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    let game_details = interactor.get_game_details().await;

    println!("{:?}", game_details);
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn devnet_add_admin() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    let admin_address = interactor.get_admin_address().await;

    interactor.add_admin(admin_address).await;
}

// We either call the deposit function or make the contract payable
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn devnet_deposit() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    let egld_amount = BigUint::from(99 * ONE_EGLD);
    interactor.deposit(egld_amount).await;
}

#[ignore]
#[tokio::test]
// #[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn submit_bet_test() {
    // let devnet_config = Config {
    //     gateway_uri: "https://devnet-gateway.multiversx.com".to_string(),
    //     chain_type: ChainType::Real,
    // };

    let mut interactor = ContractInteract::new(Config::chain_simulator_config()).await;

    // interactor.deploy().await;
    interactor
        .add_admin(test_wallets::alice().to_address())
        .await;

    let sc_balance = BigUint::from(ONE_EGLD);
    interactor.deposit(sc_balance).await;

    interactor.new_game().await;

    let bob_address = interactor
        .interactor
        .register_wallet(test_wallets::bob())
        .await;

    let bob_bet = BigUint::from(500_000u64);
    interactor.submit_bet(&bob_address, 3, bob_bet).await;

    let carol_address = interactor
        .interactor
        .register_wallet(test_wallets::carol())
        .await;

    let carol_bet = BigUint::from(900_000_000u64);
    interactor.submit_bet(&carol_address, 10, carol_bet).await;

    let dan_address = interactor
        .interactor
        .register_wallet(test_wallets::dan())
        .await;

    let dan_bet = BigUint::from(100_000u64);
    interactor.submit_bet(&dan_address, 100_000, dan_bet).await;

    println!(
        "Crash point before end is: {:?}",
        interactor.crash_point().await
    );

    interactor.end_game().await;

    println!(
        "Crash point after end is: {:?}",
        interactor.crash_point().await
    );

    interactor.compute_prizes().await;

    interactor.claim(bob_address).await;
    interactor.claim(carol_address).await;

    // NOTE: This will throw an expected panic
    // interactor.claim(dan_address).await;
}
