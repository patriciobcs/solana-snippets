use solana_program::{entrypoint::ProgramResult, account_info::{AccountInfo, next_account_info}, pubkey::Pubkey, msg};

/// Program state handler.
pub struct Processor {}
impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _input: &[u8],
    ) -> ProgramResult {
        msg!("Generic Instruction");

        Ok(())
    }
}