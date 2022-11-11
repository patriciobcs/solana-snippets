use solana_program::{rent::Rent, sysvar::Sysvar, entrypoint::ProgramResult};

/// Program state handler.
pub struct Processor {}
impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _input: &[u8],
    ) -> ProgramResult {
        msg!("Generic Instruction");

        Ok(())
    }
}
