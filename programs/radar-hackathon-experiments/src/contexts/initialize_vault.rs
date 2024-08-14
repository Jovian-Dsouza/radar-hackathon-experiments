use anchor_lang::prelude::*;
pub use crate::constants::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    // token_interface::{TokenInterface, TransferChecked},
    token::{Token, Mint, TokenAccount, TransferChecked, transfer_checked}
};

#[derive(Accounts)]
pub struct InitializeVault<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [VAULT_SEED],
        bump,
        payer=signer,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub ata: Account<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, amount: u64) -> Result<()>{
        let transfer_accounts = TransferChecked {
            from: self.ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                transfer_accounts
            ),
            amount,
            self.mint.decimals
        )
    }
    
} 