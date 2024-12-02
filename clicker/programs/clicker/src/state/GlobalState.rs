use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub total_global_cookies: u64,
    pub base_click_value: u64,
    pub total_players: u64,
    pub bump: u8,
}

impl GlobalState {
    pub const INIT_SPACE: usize = 8 + 8 + 8 + 1;
}
