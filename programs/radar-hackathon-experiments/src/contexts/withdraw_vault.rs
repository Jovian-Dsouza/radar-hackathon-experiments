use anchor_lang::prelude::*;
pub use crate::constants::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, Mint, TokenAccount, TransferChecked, transfer_checked}
};

#[derive(Accounts)]
pub struct WithdrawVault<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump,
        token::mint = mint,
        token::authority = vault,
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

impl<'info> WithdrawVault<'info> {
    pub fn withdraw_vault(&mut self, bump: u8) -> Result<()>{
        let transfer_accounts = TransferChecked {
            to: self.ata.to_account_info(),
            mint: self.mint.to_account_info(),
            from: self.vault.to_account_info(),
            authority: self.vault.to_account_info(),
        };
        let amount  = self.vault.amount;

        let signer: &[&[&[u8]]] = &[&[VAULT_SEED, &[bump]]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                transfer_accounts,
                signer
            ),
            amount,
            self.mint.decimals
        )
    }
    
} 