use anchor_lang::prelude::*;
use crate::error::MultisigError;
use crate::state::multisig_config::Multisig;
use crate::state::proposal::Proposal;

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(
        seeds = [b"multisig", creator.key().as_ref()],
        bump = multisig.bump
    )]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    pub member: Signer<'info>,

    #[account(mut)]
    pub creator: SystemAccount<'info>,
}

impl<'info> VoteOnProposal<'info> {
    pub fn vote_on_proposal(&mut self) -> Result<()> {
        let proposal = &mut self.proposal;
        let multisig = & self.multisig;
        let signer = & self.member;
        
        require!(
            multisig.members.contains(signer.key),
            MultisigError::NotAMember
        );

        // Ensure the member has not already voted
        require!(
            !proposal.voters.contains(signer.key),
            MultisigError::AlreadyVoted
        );

        proposal.votes += 1;
        proposal.voters.push(*signer.key);

        Ok(())
    }
}
