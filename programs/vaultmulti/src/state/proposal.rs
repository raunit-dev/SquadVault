use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub id: u64,
    pub amount: u64,
    pub recipient: Pubkey,
    pub votes: u64,
    pub executed: bool,
    #[max_len(10)]
    pub voters: Vec<Pubkey>,
    pub bump: u8,
}