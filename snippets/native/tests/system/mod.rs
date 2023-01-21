#![cfg(feature = "test-sbf")]

mod transfer_sol;

use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;
use solana_client::{test_instruction, test, call_instruction, get_program_context};

test_instruction!(no_accounts, solana_native::get_rent);

test_instruction!(no_accounts, solana_native::get_clock);

test_instruction!(single_readonly_account, solana_native::get_accounts);

test_instruction!(single_readonly_account, solana_native::get_account);

test_instruction!(single_readonly_account, solana_native::pack_account);

test_instruction!(single_readonly_account, solana_native::unpack_account);

test_instruction!(single_readonly_account, solana_native::check_rent);