use anchor_lang::prelude::*;

declare_id!("8KYa4f95635EoVQzwyTRGiWcqTkbd2CPGVVbeReyjWqr");

#[program]
pub mod radar_hackathon_experiments {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
