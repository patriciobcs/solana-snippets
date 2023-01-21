#![cfg(feature = "test-sbf")]

use anchor_lang::{InstructionData, ToAccountMetas};
use solana_client::{call_instruction, get_anchor_program_context, test};
use solana_program_test::*;
use solana_sdk::signature::Signer;

test!(get_bump => {
    let mut context = get_anchor_program_context!(solana_anchor);
    call_instruction!(context, solana_sdk::instruction::Instruction {
				program_id: solana_anchor::id(),
				accounts: solana_anchor::accounts::GetBump {
						user: context.payer.pubkey(),
						__pda__: solana_sdk::pubkey::Pubkey::find_program_address(&[b"__pda__"], &solana_anchor::id()).0,
						system_program: anchor_lang::solana_program::system_program::id(),
				}.to_account_metas(None),
				data: solana_anchor::instruction::GetBump {}.data(),
    }; signed by [&context.payer]);
});
