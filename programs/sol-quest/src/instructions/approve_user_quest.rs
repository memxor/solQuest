use anchor_lang::prelude::*;
use crate::constants::{MATE_SEED, ADMIN_SEED};
use crate::state::{ status::Status, admin::Admin, mate::Mate };

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