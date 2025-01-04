use anchor_lang::prelude::*;
use instruction::*;

mod error;
mod instructions;
mod state;

declare_id!("EqSbwFUjVABAMjfEAtokyqq3TeWjUYco9ztP7bz6bd27");

#[program]
pub mod lending {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidaton_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        process_init_bank(ctx, liquidaton_threshold, max_ltv);

        Ok(())
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        process_init_user(ctx, usdc_address)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        process_deposit(ctx, amount)
    }
}
