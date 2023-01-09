#![cfg(feature = "test-sbf")]

mod utils;
// use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use utils::token::{get_mint, mint_tokens, create_associated_token_account, create_mint};

test!(token_burn => {
	let mut context = get_program_context!(snippets);

	let mint = Keypair::new();
	let authority = context.payer.pubkey(); 
	let amount = 100000;

	create_mint(&mut context, &mint, &authority, Some(&authority)).await.unwrap();

	let ata = create_associated_token_account(&mut context, &authority, &mint.pubkey()).await.unwrap();

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

// test!(token_init => {
// 	let mut context = get_program_context!(snippets);

// 	let mint = Keypair::new();
// 	let authority = context.payer.pubkey(); 

// 	create_mint(&mut context, &mint, &authority, Some(&authority)).await.unwrap();

// 	let token_account = Keypair::new();

// 	call_instruction!(context, snippets::instruction::token_init(
// 				&snippets::id(), 
// 				&token_account.pubkey(), 
// 				&mint.pubkey(), 
// 				&authority,
// 				&solana_program::sysvar::rent::id(),
// 				&spl_token::id()
// 			).unwrap(); signed by [&context.payer]
// 	);
// });