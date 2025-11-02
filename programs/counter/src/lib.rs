use anchor_lang::prelude::*;

declare_id!("EF4Zr3vXtAwcfQR2EeGqiGeQ5jcfXXGXJKcdkvjHQPJ5");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
