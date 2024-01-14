use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum MateRole 
{
    Bronze,
    Silver,
    Gold,
    Platinum
}