use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::Bank;  
#[derive(Accounts)]
pub struct InitBank<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Bank::INIT_SPACE,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        init,
        token::mint = mint,
    )]
}