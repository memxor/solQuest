use anchor_lang::prelude::*;
use crate::constants::MATE_SEED;
use crate::state::{ socials::Social, mate::Mate, quest::Quest };

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