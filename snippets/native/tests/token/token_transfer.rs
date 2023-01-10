#![cfg(feature = "test-sbf")]

use solana_program_test::tokio;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::program_pack::Pack;
use spl_token::state::Account;
use crate::utils::token::{get_mint, mint_tokens, create_associated_token_account, create_mint, create_mint_token_pair};
use crate::utils::system::{create_account};
use crate::{test, call_instruction, get_program_context};

test!(token_transfer => {
	let mut context = get_program_context!(snippets);

	let amount = 100000;

	let (mint, from_ata) = create_mint_token_pair(&mut context, amount).await.unwrap();

	let to = Pubkey::new_unique();

	let to_ata = create_associated_token_account(&mut context, &to, &mint.pubkey()).await.unwrap(); 

	call_instruction!(context, snippets::instruction::token_transfer(
			&snippets::id(), 
			&from_ata, 
			&to_ata, 
			&context.payer.pubkey(), 
			&spl_token::id(),
			amount
		).unwrap(); signed by [&context.payer]
	);
});