use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MultisigConfig {
    pub min_threshold: u64,
    pub max_expiry: u64,
    pub proposal_count: u64,
    pub bump: u8,
}