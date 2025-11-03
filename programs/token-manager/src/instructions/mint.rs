use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, Token2022, TokenAccount},
};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = mint_authority,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
        associated_token::token_program = token_program,
    )]
    pub associated_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintToken<'info> {
    pub fn mint_token(&mut self, amount: u64) -> Result<()> {
        let mint_to_acc = MintTo {
            mint: self.mint_account.to_account_info(),
            to: self.associated_token_account.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };

        let cpi = CpiContext::new(
            self.token_program.to_account_info(), mint_to_acc);

        mint_to(cpi, amount)?;

        msg!(
            "Token minted successfully. Mint: {:?}\n, to: {:?}\n, amount {}\n",
            self.mint_account.to_account_info(),
            self.associated_token_account.to_account_info(),
            amount
        );

        Ok(())
    }
}
