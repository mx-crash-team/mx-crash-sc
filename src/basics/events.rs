#[multiversx_sc::module]
pub trait EventsModule {
    #[event("started_game")]
    fn started_game_event(&self, #[indexed] game_nonce: u32);

    #[event("ended_game")]
    fn ended_game_event(&self, #[indexed] crash_point: u32, #[indexed] game_nonce: u32);

    #[event("user_bet")]
    fn user_bet_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] bet: &BigUint,
        #[indexed] cash_out: u32,
    );

    #[event("winner_announcement")]
    fn winner_announcement_event(
        &self,
        #[indexed] winner: &ManagedAddress,
        #[indexed] prize: &BigUint,
        #[indexed] game_nonce: u32,
    );
}
