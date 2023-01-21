pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

macro_rules! instruction {
    ($name:ident($($arg:ident: $ty:ty),*)) => {
        pub fn $name(
            $($arg: $ty),*
        ) -> Result<()> {
            return instructions::$name::processor(
                $($arg),*
            );
        }
    };
}

#[program]
pub mod anchor {
    use super::*;

    instruction!(initialize(ctx: Context<Initialize>));
}