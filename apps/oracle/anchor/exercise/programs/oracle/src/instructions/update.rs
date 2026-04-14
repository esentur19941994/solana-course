use anchor_lang::prelude::*;
use crate::state::Oracle; // Импортируем нашу структуру из state.rs

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        has_one = owner // КРИТИЧНО: проверяет, что oracle.owner == owner.key
    )]
    pub oracle: Account<'info, Oracle>,
    
    pub owner: Signer<'info>, // Гарантирует наличие подписи владельца
}

pub fn update(ctx: Context<Update>, price: u64) -> Result<()> {
    let oracle = &mut ctx.accounts.oracle;
    oracle.price = price; // Обновляем данные
    Ok(())
}