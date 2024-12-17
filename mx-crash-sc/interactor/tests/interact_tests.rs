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
    interactor.new_game().await;

    for _i in 0..2 {
        let address = generate_wallet();
        interactor.submit_bet(address).await;
    }
    interactor.end_game().await;
    interactor.compute_prizes().await;
}

fn generate_wallet() -> Address {
    let mnemonic = Wallet::generate_mnemonic();
    let (private_key_str, public_key_str) = Wallet::get_wallet_keys_mnemonic(mnemonic.to_string());
    let wallet = Wallet::from_private_key(&private_key_str).unwrap();
    let address = wallet.to_address();

    let concatenated_keys = format!("{}{}", private_key_str, public_key_str);
    let hex_decoded_keys = hex::decode(concatenated_keys).unwrap();
    let _json_result = Wallet::encrypt_keystore(
        hex_decoded_keys.as_slice(),
        &address,
        &public_key_str,
        &Wallet::get_keystore_password(),
    );
    address
}
