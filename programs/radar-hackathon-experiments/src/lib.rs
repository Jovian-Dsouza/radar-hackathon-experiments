use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod constants {
    pub const VAULT_SEED: &[u8] = b"vault";
}

declare_id!("8KYa4f95635EoVQzwyTRGiWcqTkbd2CPGVVbeReyjWqr");

#[program]
pub mod radar_hackathon_experiments {
    use initialize_vault::InitializeVault;

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, amount: u64) -> Result<()> {
        ctx.accounts.initialize_vault(amount)
    }

    pub fn withdraw_vault(ctx: Context<WithdrawVault>) -> Result<()> {
        let bump = ctx.bumps.vault;
        ctx.accounts.withdraw_vault(bump)
    }
}
