use anchor_lang::prelude::*;
use crate::constants::MATE_SEED;
use crate::state::{ mate::Mate, mate_role::MateRole };

pub fn initialize_user(ctx: Context<InitializeUser>, nft_mint: Pubkey) -> Result<()> 
{
    let user = &mut ctx.accounts.user;

    user.authority = ctx.accounts.signer.key();
    user.mate_nft = nft_mint;
    user.mate_joined_date = Clock::get().unwrap().unix_timestamp as i64;
    user.quest_completed_by_mate = Vec::new();
    user.mate_role = MateRole::Bronze;
    user.socials = Vec::new();

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUser<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, 
        payer = signer, 
        seeds = [MATE_SEED, signer.key().as_ref()], 
        bump, 
        space = Mate::LEN)]
    pub user: Account<'info, Mate>,

    pub system_program: Program<'info, System>
}