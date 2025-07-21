use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VoteState {
    pub has_permission: bool,
    pub vote_count: u64,
    pub bump: u8,
}

