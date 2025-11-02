use anchor_lang::prelude::*;

declare_id!("77dFMte18dpEJwG6N4VRr8qqHFYe3hXzbUU8q9KvQNhH");

#[program]
pub mod sol_mind_sdk {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
