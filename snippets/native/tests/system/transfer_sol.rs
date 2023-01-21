#![cfg(feature = "test-sbf")]

use solana_sdk::signature::Signer;
use solana_program_test::tokio;
use solana_client::utils::system::{get_funded_keypair};
use solana_client::{test, call_instruction, get_program_context};

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