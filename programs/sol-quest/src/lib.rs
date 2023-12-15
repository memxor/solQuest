use anchor_lang::prelude::*;

declare_id!("4rByWqQnzNL3Zrpk6sgF22SwZCCzqc7oNP2HGHUK2iu3");

pub const QUEST_REQUIRED_FOR_SILVER: i8 = 5; // Number of completed quests required for silver
pub const QUEST_REQUIRED_FOR_GOLD: i8 = 10; // Number of completed quests required for gold
pub const QUEST_REQUIRED_FOR_PLATINUM: i8 = 15; // Number of completed quests required for platinum
pub const MATE_SEED: &[u8; 4] = b"Mate"; // Mate seed

#[program]
pub mod sol_quest 
{
    use super::*;

    // Initialize a new user
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

    // Add a completed quest to user account
    pub fn add_completed_quest(ctx: Context<AddCompletedQuest>, quest_id: i8) -> Result<()> 
    {
        let user = &mut ctx.accounts.user;

        user.quest_completed_by_mate.push(quest_id);

        if user.quest_completed_by_mate.len() as i8 >= QUEST_REQUIRED_FOR_PLATINUM
        {
            user.mate_role = MateRole::Platinum;
        }
        else if user.quest_completed_by_mate.len() as i8 >= QUEST_REQUIRED_FOR_GOLD
        {
            user.mate_role = MateRole::Gold;
        }
        else if user.quest_completed_by_mate.len() as i8 >= QUEST_REQUIRED_FOR_SILVER
        {
            user.mate_role = MateRole::Silver;
        }

        Ok(())
    }

    // Add socials to user account
    pub fn add_mate_social(ctx: Context<AddMateSocial>, socials: Vec<Social>) -> Result<()> 
    {
        let user = &mut ctx.accounts.user;

        for social in socials
        {
            user.socials.push(social);
        }

        Ok(())
    }
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

#[derive(Accounts)]
pub struct AddCompletedQuest<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut, 
        seeds = [MATE_SEED, signer.key().as_ref()], 
        bump, 
        realloc = Mate::LEN + user.quest_completed_by_mate.len() + Social::get_social_length(user.socials.clone()) + 1,
        realloc::payer = signer, 
        realloc::zero = true)]
    pub user: Account<'info, Mate>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(socials: Vec<Social>)]
pub struct AddMateSocial<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut, 
        seeds = [MATE_SEED, signer.key().as_ref()], 
        bump, 
        realloc = Mate::LEN + user.quest_completed_by_mate.len() + Social::get_social_length(user.socials.clone()) +  Social::get_social_length(socials.clone()), 
        realloc::payer = signer, 
        realloc::zero = true)]
    pub user: Account<'info, Mate>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Mate 
{
    pub authority: Pubkey,
    pub mate_nft: Pubkey,
    pub mate_joined_date: i64,
    pub quest_completed_by_mate: Vec<i8>,
    pub mate_role: MateRole,
    pub socials: Vec<Social>
}

impl Mate 
{
    pub const LEN: usize = 8 + 32 + 32 + 8 + 4 + 1 + 4 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum MateRole 
{
    Bronze,
    Silver,
    Gold,
    Platinum
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Social
{
    pub social_name: String,
    pub social_link: String
}

impl Social
{
    fn get_social_length(socials: Vec<Social>) -> usize
    {
        let size: usize = socials.iter().map(|x| x.social_name.len() + x.social_link.len() + 8).into_iter().sum();
        size
    }
}