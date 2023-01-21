#![cfg(feature = "test-sbf")]

use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_client::utils::token::create_mint_token_pair;
use solana_client::{test, call_instruction, get_program_context};

test!(token_revoke => {
	let mut context = get_program_context!(solana_native);

	let amount = 100000;
	let delegate_amount = amount;

	let (_, ata) = create_mint_token_pair(&mut context, amount).await.unwrap();

	let delegate = Pubkey::new_unique();

	call_instruction!(context, solana_native::instruction::token_approve(
			&solana_native::id(), 
			&ata, 
			&delegate, 
			&context.payer.pubkey(), 
			&spl_token::id(),
			delegate_amount
		).unwrap(); signed by [&context.payer]
	);

	call_instruction!(context, solana_native::instruction::token_revoke(
			&solana_native::id(), 
			&ata, 
			&context.payer.pubkey(), 
			&spl_token::id()
		).unwrap(); signed by [&context.payer]
	);
});