use anchor_lang::prelude::*;

declare_id!("F9WUZJNkQE3Q2QaHSeSfftV1yf99MiEcT4LazGMmZqFU");

#[program]
pub mod vaultmulti {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
