use anchor::prelude::*;

#[account]
pub struct Agent {
    pub authority: Pubkey,
    pub name: String,
    pub description: String,
    pub bump: u8,
}

impl Agent {
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 1 + (4 + 32);
}