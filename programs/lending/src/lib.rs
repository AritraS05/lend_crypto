use anchor_lang::prelude::*;
use instructions::*;  

mod instructions;
mod state;

declare_id!("ASfMy7gLvX9sCuehGtXGASznTsb2vPEFLt6UuVMZPtJX");

#[program]
pub mod lending {
    use super::*;
    pub fn init_bank(ctx: Context<InitBank>, liquidation_threshold: u64, max_ltv: u64) -> Result<()> {
        process_init_bank(ctx, liquidation_threshold, max_ltv)
    }
    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        process_init_user(ctx, usdc_address)
    }
}
