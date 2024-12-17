use multiversx_sc::imports::*;

use crate::specific::{bet::ContestantDetails, status::GameDetails};

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
    #[view(getContestantDetails)]
    fn contestant_details(&self) -> ManagedVec<ContestantDetails<Self::Api>> {
        let mut contestant_details = ManagedVec::new();
        for contestant in self.contestants().iter() {
            let bet = self.bet(&contestant).get();
            contestant_details.push(ContestantDetails {
                address: contestant,
                amount: bet.amount,
                cash_out: bet.cash_out,
            });
        }
        contestant_details
    }
}
