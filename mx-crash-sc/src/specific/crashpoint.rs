use crate::basics::{constants::HIGH_POW_OF_2, storage};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait CrashpointModule: storage::StorageModule {
    fn compute_randomness(&self) -> BigUint {
        let random_bytes = self.blockchain().get_block_random_seed();
        let hashed_bytes = self.crypto().sha256(random_bytes.as_managed_buffer());
        let rand = BigUint::from(hashed_bytes.as_managed_buffer());
        if rand.clone().rem(33u64) == 0 {
            return BigUint::zero();
        }
        rand.rem(HIGH_POW_OF_2)
    }

    fn compute_crash_point(&self) -> u32 {
        let high_power_of_2 = BigUint::from(HIGH_POW_OF_2);
        let value = self.compute_randomness();
        if value == 0u64 {
            return 0u32;
        } else {
            let crash_point = (&high_power_of_2 * 100u64 - &value) / (high_power_of_2 - value);
            crash_point.to_u64().unwrap() as u32
        }
    }
}
