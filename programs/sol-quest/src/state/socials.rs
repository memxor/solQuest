use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Social
{
    pub social_name: String,
    pub social_link: String
}

impl Social
{
    pub fn get_social_length(socials: &Vec<Social>) -> usize
    {
        let size: usize = socials.iter().map(|x| x.social_name.len() + x.social_link.len() + 8).into_iter().sum();
        size
    }
}