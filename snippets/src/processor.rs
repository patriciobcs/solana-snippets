use crate::{core::*, instruction::CustomInstruction};
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, msg, program_error::ProgramError};

// macro_rules! process {
//     ($(($instruction_name:ident, $module_name:ident),)) => {
//         match instruction {
//             $(
//                 CustomInstruction::$instruction_name {} => {
//                     msg!("Instruction: {}", stringify!($instruction_name));
//                     core::$module_name::processor()
//                 }
//             )+
//         }
//     };
// }

/// Program state handler.
pub struct Processor {}
impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = CustomInstruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)?;
    
        match instruction {
            CustomInstruction::GetRent {} => {
                msg!("Instruction: GetRent");
                get_rent::processor()
            }
            CustomInstruction::GetClock {} => {
                msg!("Instruction: GetClock");
                get_clock::processor()
            }
            CustomInstruction::GetAccounts {} => {
                msg!("Instruction: GetAccounts");
                get_accounts::processor(accounts)
            }
            CustomInstruction::GetAccount {} => {
                msg!("Instruction: GetAccount");
                get_account::processor(accounts)
            }
            CustomInstruction::PackAccount {} => {
                msg!("Instruction: PackAccount");
                pack_account::processor(accounts)
            }
            CustomInstruction::UnpackAccount {} => {
                msg!("Instruction: UnpackAccount");
                unpack_account::processor(accounts)
            }
            CustomInstruction::CheckRent {} => {
                msg!("Instruction: CheckRent");
                check_rent::processor(accounts)
            }
        }
    }
}
