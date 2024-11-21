use anchor_lang::prelude::*;

declare_id!("HJAnUjUf8az6jRRu1Mfjt7MLeQNto65qryobVKZ3mppi");

#[program]
pub mod yapper {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
