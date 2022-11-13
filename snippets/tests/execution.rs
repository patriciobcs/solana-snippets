#![cfg(feature = "test-sbf")]

mod utils;
use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;

#[tokio::test]
async fn test_get_rent() {
    test_no_accounts_instruction!(get_rent);
}

#[tokio::test]
async fn test_get_clock() {
    test_no_accounts_instruction!(get_clock);
}

#[tokio::test]
async fn gn_test_get_accounts() {
    test_single_readonly_account_instruction!(get_accounts);
}

#[tokio::test]
async fn gn_test_get_account() {
    test_single_readonly_account_instruction!(get_account);
}

#[tokio::test]
async fn gn_test_pack_account() {
    test_single_readonly_account_instruction!(pack_account);
}

#[tokio::test]
async fn gn_test_unpack_account() {
    test_single_readonly_account_instruction!(unpack_account);
}

#[tokio::test]
async fn gn_test_check_rent() {
    test_single_readonly_account_instruction!(check_rent);
}