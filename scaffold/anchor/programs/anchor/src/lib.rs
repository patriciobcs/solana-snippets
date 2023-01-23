use anchor_lang::prelude::*;

declare_id!("DvdHMjbWLdiHbskiPYqzSvhhWVnCqLuYR9vmUDojYcvs");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}