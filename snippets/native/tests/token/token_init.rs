#![cfg(feature = "test-sbf")]

// use solana_sdk::{signature::Signer, program_pack::Pack};
use solana_program_test::tokio;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::program_pack::Pack;
use spl_token::state::Account;
use crate::utils::token::{get_mint, mint_tokens, create_associated_token_account, create_mint, create_mint_token_pair};
use crate::utils::system::{create_account};
use crate::{test, call_instruction, get_program_context};

test!(token_init => {
	let mut context = get_program_context!(snippets);

	let mint = Keypair::new();
	let authority = context.payer.pubkey(); 
	let token_account = Keypair::new();

	create_mint(&mut context, &mint, &authority, Some(&authority)).await.unwrap();
	create_account(&mut context, &token_account, &spl_token::id(), Account::LEN).await.unwrap();

	call_instruction!(context, snippets::instruction::token_init(
				&snippets::id(), 
				&token_account.pubkey(), 
				&mint.pubkey(), 
				&authority,
				&solana_program::sysvar::rent::id(),
				&spl_token::id()
			).unwrap(); signed by [&context.payer]
	);
});