use multiversx_sc::imports::*;

use crate::specific::status::GameDetails;

use super::storage;

#[multiversx_sc::module]
pub trait ViewsModule: storage::StorageModule {
    #[view(getGameDetails)]
    fn get_game_details(&self) -> GameDetails {
        GameDetails {
            status: self.status().get(),
            nonce: self.game_nonce().get(),
        }
    }
}
