use anchor_lang::prelude::*;

declare_id!("4rByWqQnzNL3Zrpk6sgF22SwZCCzqc7oNP2HGHUK2iu3");

pub const QUEST_REQUIRED_FOR_SILVER: i8 = 5;
pub const QUEST_REQUIRED_FOR_GOLD: i8 = 10;
pub const QUEST_REQUIRED_FOR_PLATINUM: i8 = 15;
pub const MATE_SEED: &[u8; 4] = b"Mate";

#[program]
pub mod sol_quest 
{
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, nft_mint: Pubkey) -> Result<()> 
    {
        let user = &mut ctx.accounts.user;

        user.authority = ctx.accounts.signer.key();
        user.mate_nft = nft_mint;
        user.mate_joined_date = Clock::get().unwrap().unix_timestamp as i64;
        user.quest_completed_by_mate = Vec::new();
        user.mate_role = MateRole::Bronze;

        Ok(())
    }

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
}

#[derive(Accounts)]
pub struct InitializeUser<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, seeds = [MATE_SEED, signer.key().as_ref()], bump, space = Mate::LEN)]
    pub user: Account<'info, Mate>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddCompletedQuest<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [MATE_SEED, signer.key().as_ref()], bump, realloc = user.quest_completed_by_mate.len() + 1 + Mate::LEN, realloc::payer = signer, realloc::zero = true)]
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
    pub mate_role: MateRole
}

impl Mate 
{
    pub const LEN: usize = 8 + std::mem::size_of::<Mate>();
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum MateRole 
{
    Bronze,
    Silver,
    Gold,
    Platinum
}