use multiversx_sc_scenario::imports::*;

use mx_crash_sc::*;

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");

pub const MX_CRASH_SC_ADDRESS: TestSCAddress = TestSCAddress::new("mx-crash-sc");
pub const CODE_PATH: MxscPath = MxscPath::new("output/mx-crash-sc.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, mx_crash_sc::ContractBuilder);
    blockchain
}

pub struct MxCrashScTestState {
    world: ScenarioWorld,
}

impl MxCrashScTestState {
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
            .typed(mx_crash_sc_proxy::MxCrashScProxy)
            .init()
            .code(CODE_PATH)
            .new_address(MX_CRASH_SC_ADDRESS)
            .returns(ReturnsNewAddress)
            .run();
        assert_eq!(deployed_address, MX_CRASH_SC_ADDRESS);
        self
    }

    pub fn test_random(&mut self) {
        /*     let result = BigUint::from(100u64);
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(MX_CRASH_SC_ADDRESS)
            .typed(mx_crash_sc_proxy::MxCrashScProxy)
            .compute_crash_point()
            .returns(ExpectValue(result))
            .run();*/
    }
}
