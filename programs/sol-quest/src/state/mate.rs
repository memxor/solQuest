use anchor_lang::prelude::*;
use crate::state::{quest::Quest, mate_role::MateRole, socials::Social};

#[account]
pub struct Mate 
{
    pub authority: Pubkey,
    pub mate_nft: Pubkey,
    pub mate_joined_date: i64,
    pub quest_completed_by_mate: Vec<Quest>,
    pub mate_role: MateRole,
    pub socials: Vec<Social>
}

impl Mate 
{
    pub const LEN: usize = 8 + 32 + 32 + 8 + 4 + 1 + 4 + 8;
}