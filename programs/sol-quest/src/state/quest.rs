use anchor_lang::prelude::*;
use crate::state::status::Status;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Quest
{
    pub id: i8,
    pub deployed_url: String,
    pub transaction: String,
    pub updated_time: i64,
    pub status: Status
}

impl Quest
{
    pub fn get_quests_length(quests: &Vec<Quest>) -> usize
    {
        let size: usize = quests.iter().map(|x| x.deployed_url.len() + x.transaction.len() + 18).into_iter().sum();
        size
    }

    pub fn get_quest_length(deployed_url: &String, transaction: &String) -> usize
    {
        let size: usize = deployed_url.len() + transaction.len() + 18;
        size
    }
}