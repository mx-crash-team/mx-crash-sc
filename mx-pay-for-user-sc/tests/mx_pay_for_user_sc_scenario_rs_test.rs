use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/mx-pay-for-user-sc.mxsc.json",
        mx_pay_for_user_sc::ContractBuilder,
    );
    blockchain
}

#[test]
fn empty_rs() {}
