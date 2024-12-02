use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeGlobalState<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [b"global_state"],
        bump,
        space = GlobalState::INIT_SPACE,
    )]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGlobalState<'info> {
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        self.global_state.set_inner(GlobalState {
            total_global_cookies: 0,
            base_click_value: 1,
            total_players: 0,
            bump,
        });
        Ok(())
    }
}