#![cfg(feature = "test-sbf")]

use solana_program_test::tokio;
use solana_sdk::signature::Signer;
use solana_client::utils::token::{mint_tokens, create_mint_token_pair};
use solana_client::{test, call_instruction, get_program_context};

test!(token_burn => {
	let mut context = get_program_context!(solana_native);

	let authority = context.payer.pubkey(); 
	let amount = 100000;

	let (mint, ata) = create_mint_token_pair(&mut context, amount).await.unwrap();

	mint_tokens(&mut context, &mint.pubkey(), &ata, amount, &authority, None).await.unwrap();

	call_instruction!(context, solana_native::instruction::token_burn(
				&solana_native::id(), 
				&ata, 
				&mint.pubkey(), 
				&authority, 
				&spl_token::id(),
				amount
			).unwrap(); signed by [&context.payer]
	);
});