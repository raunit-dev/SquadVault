use anchor_lang::prelude::*;

use crate::state::multisig_config::Multisig;
use crate::state::proposal::Proposal;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub multisig: Account<'info,Multisig>,

    #[account(
        init,
        payer = creator,s
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal",multisig.key().as_ref(),&multisig.proposal_count.to_be_bytes()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        mut,
        seeds = [b"vault",multisig.key.as_ref()],
        bump = multisig.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}


impl <'info> CreateProposal <'info> {
    pub fn create_proposal(
        &mut self,
        amount: u64,
        recipient: Pubkey,
        bumps: &CreateProposalBumps,
    ) -> Result <()> {

        self.proposal.set_inner(Proposal {
            id: self.multisig.proposal_count,
            amount: amount,
            recipient: recipient,
            votes: 0,
            executed: false,
            voters: Vec::new(),
            bump: bumps.proposal,
        });

        let vault_balance = ctx.accounts.vault.to_account_info().lamports();
        require!(vault_balance >= amount, MultisigError::InsufficientVaultFunds);

        multisig.proposal_count += 1;


        Ok(())

    }
}