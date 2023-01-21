#![cfg(feature = "test-sbf")]

use solana_sdk::signature::Signer;
use solana_client::{test_anchor_instruction, test, call_instruction, get_anchor_program_context};
use solana_program_test::*;
use anchor_lang::{InstructionData, ToAccountMetas};

test_anchor_instruction!(no_accounts, solana_anchor::initialize::Initialize);