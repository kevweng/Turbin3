use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [b"player", authority.key().as_ref()],
        bump,
        space = PlayerAccount::INIT_SPACE,
    )]
    pub player_account: Account<'info, PlayerAccount>,

    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePlayer<'info> {
    pub fn initializePlayer(&mut self, bump: u8) -> Result<()> {
        self.player_account.set_inner(PlayerAccount {
            player: *self.authority.key(),
            total_cookies: 0,
            cookies_per_second: 0,
            total_clicks: 0,
        });
        Ok(())
    }
}