#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("F9WUZJNkQE3Q2QaHSeSfftV1yf99MiEcT4LazGMmZqFU");

#[program]
pub mod vaultmulti {
    use super::*;

    pub fn initialize_multisig(
        ctx: Context<InitializeMultisig>,
        members: Vec<Pubkey>,
        threshold: u64,
    ) -> Result<()> {
        ctx.accounts.initialize_multisig(&ctx.bumps, members, threshold)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        amount: u64,
        recipient: Pubkey,
    ) -> Result<()> {
        ctx.accounts.create_proposal(amount, recipient, &ctx.bumps)
    }

    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
    ) -> Result<()> {
        ctx.accounts.execute_proposal()
    }

    pub fn vote_on_proposal(
        ctx: Context<VoteOnProposal>,
    ) -> Result<()> {
        ctx.accounts.vote_on_proposal()
    }
}