use anchor_lang::prelude::*;

declare_id!("4rByWqQnzNL3Zrpk6sgF22SwZCCzqc7oNP2HGHUK2iu3");

pub const QUEST_REQUIRED_FOR_SILVER: i8 = 5; // Number of completed quests required for silver
pub const QUEST_REQUIRED_FOR_GOLD: i8 = 10; // Number of completed quests required for gold
pub const QUEST_REQUIRED_FOR_PLATINUM: i8 = 15; // Number of completed quests required for platinum
pub const MATE_SEED: &[u8; 4] = b"Mate"; // Mate seed
pub const ADMIN_SEED: &[u8; 5] = b"Admin"; // Admin seed

#[program]
pub mod sol_quest 
{
    use super::*;

    // Initialize an admin
    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> 
    {
        assert!(ctx.accounts.signer.key().to_string() == "3jyefQuStD7c2McYUKyGT4uwFKMVTm1sJzHQZo8JbQvi"); //Memxor's Dev Environment
        ctx.accounts.admin.authority = ctx.accounts.signer.key();
        Ok(())
    }

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
    pub fn add_completed_quest(ctx: Context<AddCompletedQuest>, id: i8, deployed_url: String, transaction: String) -> Result<()> 
    {
        let user = &mut ctx.accounts.user;
        let admin_user = &mut ctx.accounts.admin;

        let quest = Quest {
            id,
            deployed_url,
            transaction,
            updated_time: Clock::get().unwrap().unix_timestamp as i64,
            status: Status::SUBMITTED
        };

        user.quest_completed_by_mate.push(quest);

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

        if !admin_user.mates_submitted.contains(&user.authority)
        {
            admin_user.mates_submitted.push(user.authority.clone())
        }

        Ok(())
    }

    // Add socials to user account
    pub fn add_mate_social(ctx: Context<AddMateSocial>, socials: Vec<Social>) -> Result<()> 
    {
        let user = &mut ctx.accounts.user;

        for social in socials
        {
            let mut social_already_exists = false;
            for existing_socials in &mut user.socials
            {
                if social.social_name == existing_socials.social_name
                {
                   existing_socials.social_link = social.social_link.clone();
                   social_already_exists = true;
                }
            }

            if !social_already_exists
            {
                user.socials.push(social);
            }
        }

        Ok(())
    }

    // Approve user quest
    pub fn approve_user_quest(ctx: Context<ApproveMateQuestStatus>, quest_id: i8) -> Result<()> 
    {
        let user = &mut ctx.accounts.user;
        let admin_user = &mut ctx.accounts.admin;

        let mut mate_has_no_remaining_submitted_quest = true;
        for quest in &mut user.quest_completed_by_mate 
        {
            if quest.id == quest_id
            {
                quest.status = Status::ACCEPTED;
            }

            if quest.status == Status::SUBMITTED
            {
                mate_has_no_remaining_submitted_quest = false;
            }
        }

        if mate_has_no_remaining_submitted_quest
        {
            let index = admin_user.mates_submitted.iter().position(|x| *x == user.authority.key()).unwrap();
            admin_user.mates_submitted.remove(index);
        }

        Ok(())
    }
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
#[instruction(id: i8, deployed_url: String, transaction: String)]
pub struct AddCompletedQuest<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut, 
        seeds = [MATE_SEED, user.authority.key().as_ref()], 
        bump, 
        realloc = Mate::LEN + user.quest_completed_by_mate.len() + Social::get_social_length(&user.socials) + Quest::get_quests_length(&user.quest_completed_by_mate) + Quest::get_quest_length(&deployed_url, &transaction) ,
        realloc::payer = signer, 
        realloc::zero = true)]
    pub user: Account<'info, Mate>,

    #[account(mut, seeds = [ADMIN_SEED, admin.authority.key().as_ref()], bump, realloc = Admin::LEN + (admin.mates_submitted.len() * 32) + 32, realloc::payer = signer, realloc::zero = true)]
    pub admin: Account<'info, Admin>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ApproveMateQuestStatus<'info>
{
    #[account(mut, address = admin.authority)]
    pub signer: Signer<'info>,

    #[account(
        mut, 
        seeds = [MATE_SEED, user.authority.key().as_ref()], 
        bump)]
    pub user: Account<'info, Mate>,

    #[account(mut, seeds = [ADMIN_SEED, admin.authority.key().as_ref()], bump)]
    pub admin: Account<'info, Admin>,

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
        seeds = [MATE_SEED, user.authority.key().as_ref()], 
        bump, 
        realloc = Mate::LEN + user.quest_completed_by_mate.len() + Social::get_social_length(&user.socials) + Quest::get_quests_length(&user.quest_completed_by_mate) + Social::get_social_length(&socials), 
        realloc::payer = signer, 
        realloc::zero = true)]
    pub user: Account<'info, Mate>,

    pub system_program: Program<'info, System>
}

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum MateRole 
{
    Bronze,
    Silver,
    Gold,
    Platinum
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Status 
{
    SUBMITTED,
    ACCEPTED
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Social
{
    pub social_name: String,
    pub social_link: String
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Quest
{
    pub id: i8,
    pub deployed_url: String,
    pub transaction: String,
    updated_time: i64,
    status: Status
}

impl Social
{
    fn get_social_length(socials: &Vec<Social>) -> usize
    {
        let size: usize = socials.iter().map(|x| x.social_name.len() + x.social_link.len() + 8).into_iter().sum();
        size
    }
}

impl Quest
{
    fn get_quests_length(quests: &Vec<Quest>) -> usize
    {
        let size: usize = quests.iter().map(|x| x.deployed_url.len() + x.transaction.len() + 18).into_iter().sum();
        size
    }

    fn get_quest_length(deployed_url: &String, transaction: &String) -> usize
    {
        let size: usize = deployed_url.len() + transaction.len() + 18;
        size
    }
}