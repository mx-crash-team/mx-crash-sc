use crate::basics::{constants::HIGH_POW_OF_2, storage};

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait CrashpointModule: storage::StorageModule {
    fn compute_randomness(&self) -> u64 {
        let random_bytes = self.blockchain().get_block_random_seed();
        let rand = BigUint::from(random_bytes.as_managed_buffer());
        let mut msb = [0u8; 7];
        let _ = rand.to_bytes_be_buffer().load_slice(0, &mut msb);
        BigUint::from_bytes_be_buffer(&ManagedBuffer::from(&msb))
            .to_u64()
            .unwrap()
    }

    fn compute_crash_point(&self) -> u32 {
        let value = self.compute_randomness();
        if value.rem(33u64) == 1 {
            return 100u32;
        } else {
            let crash_point = (HIGH_POW_OF_2 * 100u64 - value) / (HIGH_POW_OF_2 - value);
            crash_point as u32
        }
    }
}
