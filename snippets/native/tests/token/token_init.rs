#![cfg(feature = "test-sbf")]

// use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::program_pack::Pack;
use spl_token::state::Account;
use solana_client::utils::token::create_mint;
use solana_client::utils::system::create_account;
use solana_client::{test, call_instruction, get_program_context};

test!(token_init => {
	let mut context = get_program_context!(solana_native);

	let mint = Keypair::new();
	let authority = context.payer.pubkey(); 
	let token_account = Keypair::new();

	create_mint(&mut context, &mint, &authority, Some(&authority)).await.unwrap();
	create_account(&mut context, &token_account, &spl_token::id(), Account::LEN).await.unwrap();

	call_instruction!(context, solana_native::instruction::token_init(
				&solana_native::id(), 
				&token_account.pubkey(), 
				&mint.pubkey(), 
				&authority,
				&solana_program::sysvar::rent::id(),
				&spl_token::id()
			).unwrap(); signed by [&context.payer]
	);
});