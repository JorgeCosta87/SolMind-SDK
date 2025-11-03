use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_interface::{
    token_metadata_initialize, Mint, Token2022, TokenMetadataInitialize,
};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct TokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
#[instruction(args: TokenMetadataArgs, decimals: u8)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = payer,
        mint::freeze_authority = payer,
        extensions::metadata_pointer::authority = payer,
        extensions::metadata_pointer::metadata_address = mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateMint<'info> {
    pub fn create_mint(&mut self, args: TokenMetadataArgs) -> Result<()> {
        let TokenMetadataArgs { name, symbol, uri } = args;

        let token_metadata = TokenMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            uri: uri.clone(),
            ..Default::default()
        };

        // The 4 bytes come from the TLV
        let data_len = 4 + token_metadata.get_packed_len().unwrap_or(0);

        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(data_len as usize);

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.payer.to_account_info(),
                    to: self.mint.to_account_info(),
                },
            ),
            lamports,
        )?;

        token_metadata_initialize(
            CpiContext::new(
                self.token_program.to_account_info(),
                TokenMetadataInitialize {
                    program_id: self.token_program.to_account_info(),
                    mint: self.mint.to_account_info(),
                    metadata: self.mint.to_account_info(),
                    mint_authority: self.payer.to_account_info(),
                    update_authority: self.payer.to_account_info(),
                },
            ),
            name,
            symbol,
            uri,
        )?;

        Ok(())
    }
}
