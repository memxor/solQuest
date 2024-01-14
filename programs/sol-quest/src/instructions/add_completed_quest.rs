use anchor_lang::prelude::*;
use crate::constants::{ MATE_SEED, ADMIN_SEED, QUEST_REQUIRED_FOR_PLATINUM, QUEST_REQUIRED_FOR_GOLD, QUEST_REQUIRED_FOR_SILVER };
use crate::state::{ quest::Quest, status::Status, mate_role::MateRole, admin::Admin, mate::Mate, socials::Social };

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