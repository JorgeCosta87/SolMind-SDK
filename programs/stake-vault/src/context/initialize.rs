use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use anchor_spl::{
    token_interface::{
        Mint,
        TokenInterface,
        TokenMetadataInitialize,
        token_metadata_initialize,
        SetAuthority,
        set_authority,
    },
};

use spl_token_2022::instruction::AuthorityType;
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

use crate::state::StakeConfig;


#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct TokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

#[derive(Accounts)]
#[instruction(args: TokenMetadataArgs)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub deposit_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        mint::decimals = args.decimals,
        mint::authority = admin,
        mint::freeze_authority = admin,
        extensions::metadata_pointer::authority = admin,
        extensions::metadata_pointer::metadata_address = reward_mint,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        space = 8 + StakeConfig::INIT_SPACE,
        payer = payer,
        seeds = [b"stake_config", admin.key().as_ref()],
        bump,
    )]
    pub stake_config: Account<'info, StakeConfig>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, base_yield_rate: u64, lock_durations: Vec<u64>, bumps: &InitializeBumps) -> Result<()>{
        self.stake_config.set_inner(StakeConfig {
            admin: self.admin.key(),
            deposit_mint: self.deposit_mint.key(),
            reward_mint: self.reward_mint.key(),
            base_yield_rate,
            lock_durations,
            total_staked: 0,
            total_positions: 0,
            bump: bumps.stake_config
        });

        Ok(())
    }

    pub fn create_mint(&mut self, args: TokenMetadataArgs) -> Result<()> {
        let token_metadata = TokenMetadata {
            name: args.name.clone(),
            symbol: args.symbol.clone(),
            uri: args.uri.clone(),
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
                    to: self.reward_mint.to_account_info(),
                },
            ),
            lamports,
        )?;

        token_metadata_initialize(
            CpiContext::new(
                self.token_program.to_account_info(),
                TokenMetadataInitialize {
                    program_id: self.token_program.to_account_info(),
                    mint: self.reward_mint.to_account_info(),
                    metadata: self.reward_mint.to_account_info(),
                    mint_authority: self.admin.to_account_info(),
                    update_authority: self.admin.to_account_info(),
                },
            ),
            args.name,
            args.symbol,
            args.uri,
        )?;

        set_authority(
            CpiContext::new(
                self.token_program.to_account_info(),
                SetAuthority {
                    current_authority: self.admin.to_account_info(),
                    account_or_mint: self.reward_mint.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            Some(self.stake_config.key()),
        )?;

        set_authority(
            CpiContext::new(
                self.token_program.to_account_info(),
                SetAuthority {
                    current_authority: self.admin.to_account_info(),
                    account_or_mint: self.reward_mint.to_account_info(),
                },
            ),
            AuthorityType::FreezeAccount,
            Some(self.stake_config.key()),
        )?;

        Ok(())
    }
}

