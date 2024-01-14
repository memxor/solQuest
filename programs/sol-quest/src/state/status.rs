use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Status 
{
    SUBMITTED,
    ACCEPTED
}