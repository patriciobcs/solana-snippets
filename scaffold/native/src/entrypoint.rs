//! Program entrypoint

use {
	crate::{error::CustomError, processor::Processor},
	solana_program::{
			account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, program_error::PrintProgramError
	},
};

entrypoint!(process_instruction);
fn process_instruction(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	instruction_data: &[u8],
) -> ProgramResult {
	if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
			// catch the error so we can print it
			error.print::<CustomError>();
			return Err(error);
	}
	Ok(())
}
