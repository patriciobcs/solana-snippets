#![cfg(feature = "test-sbf")]

mod token;
use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;
use solana_client::utils::system::{get_funded_keypair};
use solana_client::{test_instruction, test, call_instruction, get_program_context};

test_instruction!(no_accounts, solana_native::get_rent);

test_instruction!(no_accounts, solana_native::get_clock);

test_instruction!(single_readonly_account, solana_native::get_accounts);

test_instruction!(single_readonly_account, solana_native::get_account);

test_instruction!(single_readonly_account, solana_native::pack_account);

test_instruction!(single_readonly_account, solana_native::unpack_account);

test_instruction!(single_readonly_account, solana_native::check_rent);

test!(transfer_sol => {
	let mut context = get_program_context!(solana_native);

	let receiver = get_funded_keypair(&mut context).await.unwrap();

	call_instruction!(context, solana_native::instruction::transfer_sol(
				&solana_native::id(), 
				&context.payer.pubkey(), 
				&receiver.pubkey(), 
				&solana_program::system_program::id(), 
				100000
			).unwrap(); signed by [&context.payer]
	);

	call_instruction!(context, solana_native::instruction::transfer_sol(
			&solana_native::id(), 
			&context.payer.pubkey(), 
			&receiver.pubkey(), 
			&solana_program::system_program::id(), 
			100000
		).unwrap(); signed by [&context.payer]
	);
});