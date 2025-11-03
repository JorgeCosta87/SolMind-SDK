use anchor_lang::prelude::*;

declare_id!("9SwkHfBurrRq7Vw5zQ1QErAatwgdqQdaENDt6saCu8SN");

mod instructions;

pub use instructions::*;

#[program]
pub mod token_mint {
    use super::*;

    #[allow(unused_variables)]
    pub fn create_mint(
        ctx: Context<CreateMint>,
        args: TokenMetadataArgs,
        decimals: u8,
    ) -> anchor_lang::Result<()> {
        ctx.accounts.create_mint(args)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> anchor_lang::Result<()> {
        ctx.accounts.mint_token(amount)
    }
}

// Sources
// https://github.com/solana-developers/program-examples/tree/main/tokens/token-2022/nft-meta-data-pointer
// https://github.com/solana-developers/program-examples/tree/main/tokens/token-2022/metadata/anchor/programs/metadata
