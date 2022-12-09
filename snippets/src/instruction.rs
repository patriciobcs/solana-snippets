use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{instruction::{Instruction, AccountMeta}, program_error::ProgramError, pubkey::Pubkey};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CustomInstruction  {
	CheckRent {},
	GetAccount {},
	GetAccounts {},
	GetClock {},
	GetRent {},
	PackAccount {},
	// TODO: TransferSol
	UnpackAccount {},
}

pub enum AT {
	R,
	W,
	RS,
	WS,
}

#[macro_export]
macro_rules! accounts {
		() => { Vec::new() };
    ( $( $account:expr, $type:expr ),* ) => {
        {
            let mut temp_accounts = Vec::new();
            $(
								match $type {
									AT::R => temp_accounts.push(AccountMeta::new_readonly($account, false)),
									AT::W => temp_accounts.push(AccountMeta::new($account, false)),
									AT::RS => temp_accounts.push(AccountMeta::new_readonly($account, true)),
									AT::WS => temp_accounts.push(AccountMeta::new($account, true)),
								}
						)*
            temp_accounts
        }
    };
}

#[macro_export]
macro_rules! instruction_handler {
		($instruction_name:ident, $fn_name:ident $(, ($account:ident, $type:expr))*) => {
			pub fn $fn_name(program_id: &Pubkey $(, $account: &Pubkey)*) -> Result<Instruction, ProgramError> {
				let accounts = accounts!($(*$account, $type)*);

				Ok(Instruction {
						program_id: *program_id,
						accounts,
						data: CustomInstruction::$instruction_name {}.try_to_vec().unwrap(),
				})
			}
		};
}

#[macro_export]
macro_rules! instruction {
		(no_accounts, $instruction_name:ident, $fn_name:ident) => {
			instruction_handler!($instruction_name, $fn_name);
		};
		(single_readonly_account, $instruction_name:ident, $fn_name:ident) => {
			instruction_handler!($instruction_name, $fn_name, (account, AT::R));
		};
}

instruction!(no_accounts, GetRent, get_rent);

instruction!(no_accounts, GetClock, get_clock);

instruction!(single_readonly_account, GetAccounts, get_accounts);

instruction!(single_readonly_account, GetAccount, get_account);

instruction!(single_readonly_account, PackAccount, pack_account);

instruction!(single_readonly_account, UnpackAccount, unpack_account);

instruction!(single_readonly_account, CheckRent, check_rent);