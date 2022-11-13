use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{instruction::{Instruction, AccountMeta}, program_error::ProgramError, pubkey::Pubkey};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CustomInstruction  {
		GetRent {},
		GetClock {},
		GetAccounts {},
		GetAccount {},
		PackAccount {},
		UnpackAccount {},
		CheckRent {},
}

pub fn no_accounts_instruction(program_id: &Pubkey, instruction: CustomInstruction) -> Result<Instruction, ProgramError> {
	let accounts = vec![];

	Ok(Instruction {
			program_id: *program_id,
			accounts,
			data: instruction.try_to_vec().unwrap(),
	})
}

pub fn single_readonly_account_instruction(program_id: &Pubkey, instruction: CustomInstruction, account: &Pubkey) -> Result<Instruction, ProgramError> {
	let mut accounts = vec![];
	accounts.push(AccountMeta::new_readonly(*account, false));

	Ok(Instruction {
			program_id: *program_id,
			accounts,
			data: instruction.try_to_vec().unwrap(),
	})
}

pub fn grent(program_id: &Pubkey) -> Result<Instruction, ProgramError> {
	no_accounts_instruction(program_id, CustomInstruction::GetRent {})
}

pub fn gclock(program_id: &Pubkey) -> Result<Instruction, ProgramError> {
	no_accounts_instruction(program_id, CustomInstruction::GetClock {})
}

pub fn gaccs(program_id: &Pubkey, account: &Pubkey) -> Result<Instruction, ProgramError> {
	single_readonly_account_instruction(program_id, CustomInstruction::GetAccounts {}, account)
}

pub fn gacc(program_id: &Pubkey, account: &Pubkey) -> Result<Instruction, ProgramError> {
	single_readonly_account_instruction(program_id, CustomInstruction::GetAccount {}, account)
}

pub fn apack(program_id: &Pubkey, account: &Pubkey) -> Result<Instruction, ProgramError> {
	single_readonly_account_instruction(program_id, CustomInstruction::PackAccount {}, account)
}

pub fn aunpack(program_id: &Pubkey, account: &Pubkey) -> Result<Instruction, ProgramError> {
	single_readonly_account_instruction(program_id, CustomInstruction::UnpackAccount {}, account)
}

pub fn chrent(program_id: &Pubkey, account: &Pubkey) -> Result<Instruction, ProgramError> {
	single_readonly_account_instruction(program_id, CustomInstruction::CheckRent {}, account)
}