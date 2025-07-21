use anchor_lang::prelude::*;
use crate::error::MultisigError;
use crate::state::multisig_config::Multisig;

#[derive(Accounts)]
pub struct InitializeMultisig<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + Multisig::INIT_SPACE,
        seeds = [b"multisig",creator.key().as_ref()],
        bump
    )]
    pub multisig: Account<'info, Multisig>,
     #[account(
        seeds = [b"vault", multisig.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> InitializeMultisig <'info> {
    pub fn initialize_multisig(
        &mut self,
        bumps: &InitializeMultisigBumps,
        members: Vec<Pubkey>,
        threshold: u64,
    ) -> Result <()> {
        require!(threshold > 0 && threshold <= members.len() as u64,MultisigError::InvalidThreshold);
        require!(!members.is_empty(), MultisigError::NoMembers);

        self.multisig.set_inner(Multisig {
            members: members,
            threshold: threshold,
            proposal_count: 0,
            bump: bumps.multisig,
            vault_bump: bumps.vault
        });

        Ok(())

    }
}

