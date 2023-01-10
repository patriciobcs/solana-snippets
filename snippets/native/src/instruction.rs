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
	TransferSol { amount : u64 },
	TokenApprove { amount: u64 },
	TokenBurn { amount: u64 },
	TokenInit {},
	TokenRevoke {},
	TokenTransfer { amount: u64 },
}

pub enum AT {
	R,
	W,
	RS,
	WS,
}

macro_rules! account_meta {
	(R, $account:ident) => { AccountMeta::new_readonly(*$account, false)};
	(W, $account:ident) => { AccountMeta::new(*$account, false) };
	(RS, $account:ident) => { AccountMeta::new_readonly(*$account, true) };
	(WS, $account:ident) => { AccountMeta::new(*$account, true) };
}

#[macro_export]
macro_rules! accounts {
		() => { Vec::new() };
    ( $( $account:ident, $ty:ident ),* ) => {
        {
            let mut temp_accounts = Vec::new();
            $(
								temp_accounts.push(account_meta!($ty, $account));
						)*
            temp_accounts
        }
    };
}

#[macro_export]
macro_rules! instruction_handler {
		($instruction_name:ident, $fn_name:ident$(, ($account:ident, $type:ident))*$(, $arg:ident: $ty:ty)*) => {
			pub fn $fn_name(program_id: &Pubkey $(, $account: &Pubkey)* $(, $arg: $ty)*) -> Result<Instruction, ProgramError> {
				let accounts = accounts!($($account, $type),*);

				Ok(Instruction {
						program_id: *program_id,
						accounts,
						data: CustomInstruction::$instruction_name { $($arg),* }.try_to_vec().unwrap(),
				})
			}
		};
}

#[macro_export]
macro_rules! instruction {
		($instruction_name:ident, $fn_name:ident($($arg:ident: $ty:ty),*$(| $account:ident: $type:ident)*)) => {
			instruction_handler!($instruction_name, $fn_name$(, ($account, $type))*$(, $arg: $ty)*);
		};
		(no_accounts, $instruction_name:ident, $fn_name:ident($($arg:ident: $ty:ty),*)) => {
			instruction_handler!($instruction_name, $fn_name$(, $arg: $ty)*);
		};
		(single_readonly_account, $instruction_name:ident, $fn_name:ident()) => {
			instruction!($instruction_name, $fn_name(| account: R));
		};
}

instruction!(no_accounts, GetRent, get_rent());

instruction!(no_accounts, GetClock, get_clock());

instruction!(single_readonly_account, GetAccounts, get_accounts());

instruction!(single_readonly_account, GetAccount, get_account());

instruction!(single_readonly_account, PackAccount, pack_account());

instruction!(single_readonly_account, UnpackAccount, unpack_account());

instruction!(single_readonly_account, CheckRent, check_rent());

instruction!(TransferSol, transfer_sol(amount: u64 | sender: WS | receiver: W | system_program: R));

instruction!(TokenApprove, token_approve(amount: u64 | token: W | delegate: R | authority: RS | token_program: R));

instruction!(TokenBurn, token_burn(amount: u64 | token: W | mint: W | authority: RS | token_program: R));

instruction!(TokenInit, token_init(| token: W | mint: R | authority: R | rent: R | token_program: R));

instruction!(TokenRevoke, token_revoke(| token: W | authority: RS | token_program: R));

instruction!(TokenTransfer, token_transfer(amount: u64 | from: W | to: W | authority: RS | token_program: R));