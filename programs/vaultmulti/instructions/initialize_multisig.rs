use anchor_lang::prelude::*;

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

