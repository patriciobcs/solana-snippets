#![cfg(feature = "test-sbf")]

mod utils;
mod token;
use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;
use utils::system::{get_funded_keypair};

test_instruction!(no_accounts, get_rent);

test_instruction!(no_accounts, get_clock);

test_instruction!(single_readonly_account, get_accounts);

test_instruction!(single_readonly_account, get_account);

test_instruction!(single_readonly_account, pack_account);

test_instruction!(single_readonly_account, unpack_account);

test_instruction!(single_readonly_account, check_rent);

test!(transfer_sol => {
	let mut context = get_program_context!(snippets);

	let receiver = get_funded_keypair(&mut context).await.unwrap();

	call_instruction!(context, snippets::instruction::transfer_sol(
				&snippets::id(), 
				&context.payer.pubkey(), 
				&receiver.pubkey(), 
				&solana_program::system_program::id(), 
				100000
			).unwrap(); signed by [&context.payer]
	);

	call_instruction!(context, snippets::instruction::transfer_sol(
			&snippets::id(), 
			&context.payer.pubkey(), 
			&receiver.pubkey(), 
			&solana_program::system_program::id(), 
			100000
		).unwrap(); signed by [&context.payer]
	);
});