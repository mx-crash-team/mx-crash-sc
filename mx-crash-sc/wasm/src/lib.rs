// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           26
// Async Callback (empty):               1
// Total number of exported functions:  29

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    mx_crash_sc
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        withdraw => withdraw
        givePermission => give_permission
        revokePermission => revoke_permission
        setDuration => set_duration
        setInstantCrashChance => set_instant_crash_chance
        newGame => new_game
        status => status
        game_duration => game_duration
        game_nonce => game_nonce
        crash_point => crash_point
        contestants => contestants
        available_prize => available_prize
        submitBet => submit_bet
        endGame => end_game
        claim => claim
        computePrizes => compute_prizes
        getGameDetails => get_game_details
        getContestantDetails => contestant_details
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
