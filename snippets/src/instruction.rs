use std::mem::size_of;

use solana_program::{instruction::{Instruction, AccountMeta}, program_error::ProgramError, pubkey::Pubkey};

use crate::error::CustomError::InvalidInstruction;

#[cfg(feature = "serde-traits")]
use {
    crate::serialization::coption_fromstr,
    serde::{Deserialize, Serialize},
    serde_with::{As, DisplayFromStr},
};

#[repr(C)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum CustomInstruction  {
		GetRent {}
}

impl CustomInstruction {
	pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
			let (&tag, _rest) = input.split_first().ok_or(InvalidInstruction)?;

			Ok(match tag {
					0 => Self::GetRent {},
					_ => return Err(InvalidInstruction.into()),
			})
	}


	pub fn pack(&self) -> Vec<u8> {
		let mut buf = Vec::with_capacity(size_of::<Self>());
		match self {
			Self::GetRent {} => buf.push(0)
		}
		buf
	}
}

pub fn no_accounts_instruction(program_id: &Pubkey, instruction_data: CustomInstruction) -> Result<Instruction, ProgramError> {
	let data = instruction_data.pack();

	let accounts = vec![];

	Ok(Instruction {
			program_id: *program_id,
			accounts,
			data,
	})
}

pub fn single_readonly_account_instruction<T>(program_id: &Pubkey, instruction_data: CustomInstruction, account: &Pubkey) -> Result<Instruction, ProgramError> {
	let data = instruction_data.pack();

	let mut accounts = vec![];
	accounts.push(AccountMeta::new_readonly(*account, false));

	Ok(Instruction {
			program_id: *program_id,
			accounts,
			data,
	})
}

pub fn grent(program_id: &Pubkey) -> Result<Instruction, ProgramError> {
	no_accounts_instruction(program_id, CustomInstruction::GetRent {})
}
