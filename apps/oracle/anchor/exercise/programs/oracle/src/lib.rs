use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("AiGBtqEtQZ3XdroXmgmhGg698BmMszMUjPp2FX8kKk8M");

#[program]
pub mod oracle {
    pub use super::instructions::*;
    use super::*;

    pub fn init(ctx: Context<Init>, price: u64) -> Result<()> {
        instructions::init(ctx, price)?;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, price: u64) -> Result<()> {
        instructions::update(ctx, price)?; // Вызов твоей логики
        Ok(())
    }
}