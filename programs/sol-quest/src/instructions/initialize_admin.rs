use anchor_lang::prelude::*;
use crate::constants::ADMIN_SEED;
use crate::state::admin::Admin;

pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> 
{
    assert!(ctx.accounts.signer.key().to_string() == "3jyefQuStD7c2McYUKyGT4uwFKMVTm1sJzHQZo8JbQvi");
    ctx.accounts.admin.authority = ctx.accounts.signer.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [ADMIN_SEED, signer.key().as_ref()],
        bump,
        space = Admin::LEN)]
    pub admin: Account<'info, Admin>,

    pub system_program: Program<'info, System>
}