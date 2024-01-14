use anchor_lang::prelude::*;
pub mod instructions;
pub use instructions::*;
mod state;
mod constants;
use crate::state::socials::Social;

declare_id!("4rByWqQnzNL3Zrpk6sgF22SwZCCzqc7oNP2HGHUK2iu3");

#[program]
pub mod sol_quest 
{
    use super::*;

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()>
    {
        initialize_admin::initialize_admin(ctx)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, nft_mint: Pubkey) -> Result<()> 
    {
        initialize_user::initialize_user(ctx, nft_mint)
    }

    pub fn add_completed_quest(ctx: Context<AddCompletedQuest>, id: i8, deployed_url: String, transaction: String) -> Result<()> 
    {
        add_completed_quest::add_completed_quest(ctx, id, deployed_url, transaction)
    }

    pub fn add_mate_social(ctx: Context<AddMateSocial>, socials: Vec<Social>) -> Result<()> 
    {
        add_mate_social::add_mate_social(ctx, socials)
    }

    pub fn approve_user_quest(ctx: Context<ApproveMateQuestStatus>, quest_id: i8) -> Result<()> 
    {
        approve_user_quest::approve_user_quest(ctx, quest_id)
    }
}