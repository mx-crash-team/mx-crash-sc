use multiversx_sc_scenario::imports::*;

use mx_pay_for_user_sc::*;

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");

pub const MX_CRASH_SC_ADDRESS: TestSCAddress = TestSCAddress::new("mx-crash-sc");
pub const MX_PAY_FOR_USE_SC_ADDRESS: TestSCAddress = TestSCAddress::new("mx-pay-for-user-sc");
pub const CODE_PATH: MxscPath = MxscPath::new("output/mx-crash-sc.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, mx_pay_for_user_sc::ContractBuilder);
    blockchain
}

pub struct MxPayForUserScTestState {
    world: ScenarioWorld,
}

impl MxPayForUserScTestState {
    pub fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1);

        Self { world }
    }

    pub fn deploy(&mut self) -> &mut Self {
        let deployed_address = self
            .world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(mx_pay_for_user_proxy::MxPayForUserScProxy)
            .init(MX_CRASH_SC_ADDRESS)
            .code(CODE_PATH)
            .new_address(MX_PAY_FOR_USE_SC_ADDRESS)
            .returns(ReturnsNewAddress)
            .run();
        assert_eq!(deployed_address, MX_PAY_FOR_USE_SC_ADDRESS);
        self
    }
}
