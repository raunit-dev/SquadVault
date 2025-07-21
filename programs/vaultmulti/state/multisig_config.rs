use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(10)] // Max 10 members for this example
    pub members: Vec<Pubkey>,
    pub threshold: u64,
    pub proposal_count: u64,
    pub bump: u8,
    pub vault_bump: u8,
}