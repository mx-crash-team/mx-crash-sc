use multiversx_sc_snippets::{hex, imports::*};
use rust_interact::ContractInteract;

// Simple deploy test that runs on the real blockchain configuration.
// In order for this test to work, make sure that the `config.toml` file contains the real blockchain config (or choose it manually)
// Can be run with `sc-meta test`.
#[tokio::test]
//#[ignore = "run on demand, relies on real blockchain state"]
async fn deploy_test_mx_crash_sc() {
    let mut interactor = ContractInteract::new().await;

    interactor.deploy().await; //
    for _j in 0..20 {
        interactor.new_game().await;
        interactor.submit_bet().await;
        interactor.end_game().await;
        interactor.compute_prizes().await;
    }
}
