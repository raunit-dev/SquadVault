use crate::error::MultisigError;
use crate::state::multisig_config::Multisig;
use crate::state::proposal::Proposal;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(
        mut,
        close = creator
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        mut,
        seeds = [b"vault",multisig.key().as_ref()],
        bump = multisig.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub recipient: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ExecuteProposal<'info> {
    pub fn execute_proposal(&mut self) -> Result<()> {
        let proposal = &mut self.proposal;
        let multisig = &self.multisig;
        require!(!proposal.executed, MultisigError::ProposalAlreadyExecuted);

        require!(
            proposal.votes >= self.multisig.threshold,
            MultisigError::ThresholdNotMet
        );

        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            system_program::Transfer {
                from: self.vault.to_account_info(),
                to: self.recipient.to_account_info(),
            },
        );

        let seeds = &[
            b"vault".as_ref(),
            &multisig.key().to_bytes(),
            &[multisig.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        system_program::transfer(cpi_context.with_signer(signer_seeds), proposal.amount)?;

        proposal.executed = true;

        Ok(())
    }
}
