use multiversx_sc::imports::*;

use crate::specific::status::GameDetails;

use super::storage;

#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {
    #[view(getGameDetails)]
    fn get_game_details(&self) -> GameDetails {
        let game_times = self.game_times().get();
        GameDetails {
            status: self.status().get(),
            nonce: self.game_nonce().get(),
            duration: game_times.duration,
            init_moment: game_times.init_moment,
            current_timestamp: self.blockchain().get_block_timestamp(),
        }
    }
}
