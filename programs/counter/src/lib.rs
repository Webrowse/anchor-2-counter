use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod counter_program {


    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<ModifyCounter>) -> Result<()>{
        let counter = &mut ctx.accounts.counter_account;
        counter.count = counter.count.checked_add(1).ok_or(CustomError::CounterOverflow)?;
        Ok(())
    }

    pub fn decrement(ctx: Context<ModifyCounter>) -> Result<()>{
        let counter = &mut ctx.accounts.counter_account;
        require!(counter.count > 0, CustomError::CounterUnderflow);
        counter.count = counter.count.checked_sub(1).ok_or(CustomError::CounterUnderflow)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(
        init, payer = user, 
        space = 8 + 8,
        seeds = [b"counter", user.key().as_ref()],
        bump    
    )]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCounter<'info> {
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter_account: Account<'info, CounterAccount>,
    
    pub user: Signer<'info>,
}

#[account]
pub struct CounterAccount   {
    pub count: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Counter cannot be below zero.")]
    CounterUnderflow,
    #[msg("Cannot be infinitely large.")]
    CounterOverflow,
}