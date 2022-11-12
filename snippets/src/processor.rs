use crate::{core::grent, instruction::CustomInstruction};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, msg};

/// Program state handler.
pub struct Processor {}
impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = CustomInstruction::unpack(input)?;

        match instruction {
            CustomInstruction::GetRent {} => {
                msg!("Instruction: GetRent");
                grent::processor()
            }
        }
    }
}
