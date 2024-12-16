use crate::basics::storage;

use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait CrashpointModule: storage::StorageModule {
    fn compute_randomness(&self) -> BigUint {
        let random_bytes = self.blockchain().get_block_random_seed();
        let rand = BigUint::from(random_bytes.as_managed_buffer());
        let mut msb = [0u8; 7];
        let _ = rand.to_bytes_be_buffer().load_slice(0, &mut msb);
        BigUint::from_bytes_be_buffer(&ManagedBuffer::from(&msb))
    }

    fn compute_crash_point(&self) -> BigUint {
        let high_pow_of_2 = BigUint::from(2u64).pow(52u32);
        let value = self.compute_randomness();
        (&high_pow_of_2 * 100u64 - &value) / (high_pow_of_2 - value)
    }
}
