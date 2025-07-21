#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;



declare_id!("F9WUZJNkQE3Q2QaHSeSfftV1yf99MiEcT4LazGMmZqFU");

#[program]
pub mod vaultmulti {
    

    use super::*;

    pub fn Initialize(ctx: Context<InitializeMultisig>,members: Vec<Pubkey>,threshold: u64) -> Result<()> {
        ctx.accounts.Intialize_mutlisig(&ctx.bumps,members,threshold)?:
        Ok(())
    }
}
