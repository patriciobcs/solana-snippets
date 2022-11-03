use crate::state::Account;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _input: &[u8],
    ) -> ProgramResult {
        msg!("Generic Instruction");

        let account_info_iter = &mut accounts.iter();
        let custom_account_info = next_account_info(account_info_iter)?;

        let rent = Rent::get()?;

        if !rent.is_exempt(custom_account_info.lamports(), Account::LEN) {
            return Err(ProgramError::InvalidAccountData.into());
        }

        Ok(())
    }
}
