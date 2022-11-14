#![cfg(feature = "test-sbf")]

mod utils;
use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;

test_instruction!(no_accounts, get_rent);

test_instruction!(no_accounts, get_clock);

test_instruction!(single_readonly_account, get_accounts);

test_instruction!(single_readonly_account, get_account);

test_instruction!(single_readonly_account, pack_account);

test_instruction!(single_readonly_account, unpack_account);

test_instruction!(single_readonly_account, check_rent);