use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Click<'info> {
    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump = player.bump,
    )]
    pub player: Account<'info, PlayerAccount>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump = global_state.bump,
    )]
    pub global_state: Account<'info, GlobalState>,
}

impl<'info> Click<'info> {
    pub fn click(&mut self) -> Result<()> {
        //TODO: Implement click logic
        
        Ok(())
    }
}