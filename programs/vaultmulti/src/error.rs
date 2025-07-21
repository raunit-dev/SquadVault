use anchor_lang::prelude::*;

#[error_code]
pub enum MultisigError {
    #[msg("The threshold must be greater than 0 and less than or equal to the number of members.")]
    InvalidThreshold,
    #[msg("The multisig must have at least one member.")]
    NoMembers,
    #[msg("The signer is not a member of this multisig.")]
    NotAMember,
    #[msg("This member has already voted on this proposal.")]
    AlreadyVoted,
    #[msg("The voting threshold has not been met.")]
    ThresholdNotMet,
    #[msg("This proposal has already been executed.")]
    ProposalAlreadyExecuted,
    #[msg("The vault does not have enough funds for this proposal.")]
    InsufficientVaultFunds,
}