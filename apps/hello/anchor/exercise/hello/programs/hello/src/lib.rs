use anchor_lang::prelude::*;

declare_id!("FSTX2fDgRfX5hDX8oAfMiSrAGf9BDYWm6pNAunXmZPd1");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
