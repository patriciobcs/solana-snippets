use crate::{core::{system::*, token::*}, instruction::CustomInstruction};
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
            // System
            CustomInstruction::CheckRent {} => {
                msg!("Instruction: CheckRent");
                check_rent::processor(accounts)
            }
            CustomInstruction::GetAccount {} => {
                msg!("Instruction: GetAccount");
                get_account::processor(accounts)
            }
            CustomInstruction::GetAccounts {} => {
                msg!("Instruction: GetAccounts");
                get_accounts::processor(accounts)
            }
            CustomInstruction::GetClock {} => {
                msg!("Instruction: GetClock");
                get_clock::processor()
            }
            CustomInstruction::GetRent {} => {
                msg!("Instruction: GetRent");
                get_rent::processor()
            }
            CustomInstruction::PackAccount {} => {
                msg!("Instruction: PackAccount");
                pack_account::processor(accounts)
            }
            CustomInstruction::UnpackAccount {} => {
                msg!("Instruction: UnpackAccount");
                unpack_account::processor(accounts)
            }
            CustomInstruction::TransferSol { amount } => {
                msg!("Instruction: TransferSol");
                transfer_sol::processor(accounts, amount)
            }
            CustomInstruction::TokenApprove { amount } => {
                msg!("Instruction: TokenApprove");
                token_approve::processor(accounts, amount)
            }
            CustomInstruction::TokenBurn { amount } => {
                msg!("Instruction: TokenBurn");
                token_burn::processor(accounts, amount)
            }
            CustomInstruction::TokenInit { } => {
                msg!("Instruction: TokenInit");
                token_init::processor(accounts)
            }
            CustomInstruction::TokenRevoke { } => {
                msg!("Instruction: TokenRevoke");
                token_revoke::processor(accounts)
            }
            CustomInstruction::TokenTransfer { amount } => {
                msg!("Instruction: TokenTransfer");
                token_transfer::processor(accounts, amount)
            }
        }
    }
}
