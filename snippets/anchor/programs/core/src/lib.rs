pub mod instructions;

pub use instructions::*;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// macro_rules! instruction {
//     ($name:ident($($arg:ident: $ty:ty),*)) => {
//         pub fn $name(
//             $($arg: $ty),*
//         ) -> Result<()> {
//             return instructions::$name::processor(
//                 $($arg),*
//             );
//         }
//     };
// }
// instruction!(initialize(ctx: Context<Initialize>));

#[program]
pub mod anchor {
    pub use super::*;

    pub fn get_bump(ctx: Context<GetBump>) -> Result<()> {
        instructions::get_bump::processor(ctx)
    }
}