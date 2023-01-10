#![cfg(feature = "test-sbf")]

// use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use crate::utils::token::{get_mint, mint_tokens, create_associated_token_account, create_mint, create_mint_token_pair};
use crate::{test, call_instruction, get_program_context};

test!(token_burn => {
	let mut context = get_program_context!(snippets);

	let mint = Keypair::new();
	let authority = context.payer.pubkey(); 
	let amount = 100000;

	let (mint, ata) = create_mint_token_pair(&mut context, amount).await.unwrap();

	mint_tokens(&mut context, &mint.pubkey(), &ata, amount, &authority, None).await.unwrap();

	call_instruction!(context, snippets::instruction::token_burn(
				&snippets::id(), 
				&ata, 
				&mint.pubkey(), 
				&authority, 
				&spl_token::id(),
				amount
			).unwrap(); signed by [&context.payer]
	);
});