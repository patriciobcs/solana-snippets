mod instructions;
mod state;

pub use instructions::*;
pub use state::*;

use anchor_lang::prelude::*;

declare_id!("DvdHMjbWLdiHbskiPYqzSvhhWVnCqLuYR9vmUDojYcvs");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::processor(ctx)?;

        
        Ok(())
    }
}
