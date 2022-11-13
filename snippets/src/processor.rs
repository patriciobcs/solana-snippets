use crate::{core::*, instruction::CustomInstruction};
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, msg, program_error::ProgramError};

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
                grent::processor()
            }
            CustomInstruction::GetClock {} => {
                msg!("Instruction: GetClock");
                gclock::processor()
            }
            CustomInstruction::GetAccounts {} => {
                msg!("Instruction: GetAccounts");
                gaccs::processor(accounts)
            }
            CustomInstruction::GetAccount {} => {
                msg!("Instruction: GetAccount");
                gacc::processor(accounts)
            }
            CustomInstruction::PackAccount {} => {
                msg!("Instruction: PackAccount");
                apack::processor(accounts)
            }
            CustomInstruction::UnpackAccount {} => {
                msg!("Instruction: UnpackAccount");
                aunpack::processor(accounts)
            }
            CustomInstruction::CheckRent {} => {
                msg!("Instruction: CheckRent");
                chrent::processor(accounts)
            }
        }
    }
}
