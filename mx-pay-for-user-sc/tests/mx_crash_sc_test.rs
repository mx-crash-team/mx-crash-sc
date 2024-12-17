mod contract_setup;
use contract_setup::*;

#[test]
fn test_fund() {
    let mut state = MxPayForUserScTestState::new();
    state.deploy();
}
