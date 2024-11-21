use anchor::prelude::*;

#[account]
pub struct Agent {
    pub authority: Pubkey,
    pub name: String,
    pub description: String,
}