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
		GenericInstruction {}
}

impl CustomInstruction {
	pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
			let (&tag, _rest) = input.split_first().ok_or(InvalidInstruction)?;

			Ok(match tag {
					0 => Self::GenericInstruction {},
					_ => return Err(InvalidInstruction.into()),
			})
	}


	pub fn pack(&self) -> Vec<u8> {
		let mut buf = Vec::with_capacity(size_of::<Self>());
		match self {
			Self::GenericInstruction {} => buf.push(0)
		}
		buf
	}
}


pub fn generic_instruction(program_id: &Pubkey, account: &Pubkey) -> Result<Instruction, ProgramError> {
	let data = CustomInstruction::GenericInstruction{}.pack();

	let mut accounts = vec![];
	accounts.push(AccountMeta::new_readonly(*account, false));

	Ok(Instruction {
			program_id: *program_id,
			accounts,
			data,
	})
}