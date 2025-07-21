use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub proposal_id: u64,
    pub expiry: u64,
    pub result: ProposalStatus,
    pub bump: u8,
    pub active_members: [Pubkey; 5],
    pub vote: [u8; 10],
    pub created_time: u64,
}

impl Space for Proposal {
    // 8 discriminator + 8 proposal_id + 8 expiry + 1 result + 1 bump + (5*32) active_members + 10 vote + 8 created_time
    const INIT_SPACE: usize = 8 + 8 + 8 + 1 + 1 + (5 * 32) + 10 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ProposalStatus {
    Draft = 0,
    Active = 1,
    Failed = 2,
    Succeeded = 3,
    Cancelled = 4,
}