use anchor_lang::prelude::*;

#[account]
pub struct Admin 
{
    pub authority: Pubkey,
    pub mates_submitted: Vec<Pubkey>
}

impl Admin
{
    pub const LEN: usize = 8 + 32 + 4;
}