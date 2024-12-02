use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PurchaseUpgrade<'info> {
    #[account(mut)]
    pub player: Account<'info, PlayerAccount>,

    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump = player.bump,
    )]
    pub player_state: Account<'info, PlayerState>,
}

impl<'info> PurchaseUpgrade<'info> {
    pub fn purchase_upgrade(&mut self) -> Result<()> {
        Ok(())
    }
}