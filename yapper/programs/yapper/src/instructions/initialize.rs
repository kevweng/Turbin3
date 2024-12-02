use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = Agent::INIT_SPACE,
        seeds = [seed.to_le_bytes().as_ref()],
        bump
    )]
    pub agent: Account<'info, Agent>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seed: u64, authority: Pubkey, name: String, description: String) -> Result<()> {
        self.agent.set_inner(Agent {
            authority,
            name,
            description,
            seed
        });
        Ok(())
    }
}