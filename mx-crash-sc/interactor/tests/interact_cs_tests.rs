use multiversx_sc_snippets::imports::*;
use rust_interact::ContractInteract;

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.
// #[tokio::test]
// #[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
// async fn deploy_test_mx_crash_sc_cs() {
//     let mut interactor = ContractInteract::new().await;

//     interactor.deploy().await;
//     interactor.new_game().await;
// }

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn submit_bet_test() {
    let mut interactor = ContractInteract::new().await;

    interactor.deploy().await;

    let sc_balance = BigUint::from(100_000_000_000u64);
    interactor.deposit(sc_balance).await;

    interactor.new_game().await;

    let bet = BigUint::from(100_000u64);
    interactor
        .submit_bet(test_wallets::alice().to_address(), 10, bet)
        .await;
}
