mod contract_setup;
use contract_setup::*;

#[test]
fn test_fund() {
    let mut state = MxCrashScTestState::new();
    state.deploy();
    state.test_random();
}
