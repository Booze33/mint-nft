use anchor_lang::prelude::*;

declare_id!("7mXHM57iu17ivdma6HGEGmQ6oAcsn4QeFWHBcbX4eUnu");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
