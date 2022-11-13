#![cfg(feature = "test-sbf")]

#[macro_export]
macro_rules! test_no_accounts_instruction {
	($instruction_name:ident ) => {
				let pt = solana_program_test::ProgramTest::new("snippets", snippets::id(), solana_program_test::processor!(snippets::processor::Processor::process));
				let (mut banks_client, payer, recent_blockhash) = pt.start().await;
				let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
						&[snippets::instruction::$instruction_name(&snippets::id()).unwrap()],
						Some(&payer.pubkey()),
						&[&payer],
						recent_blockhash,
				);
				banks_client.process_transaction(transaction).await.unwrap();
    };
}

#[macro_export]
macro_rules! test_single_readonly_account_instruction {
	($instruction_name:ident ) => {
				let pt = solana_program_test::ProgramTest::new("snippets", snippets::id(), solana_program_test::processor!(snippets::processor::Processor::process));
				let (mut banks_client, payer, recent_blockhash) = pt.start().await;

				let account = solana_sdk::signature::Keypair::new();
				let rent = banks_client.get_rent().await.unwrap();
				let account_rent = rent.minimum_balance(snippets::state::__Account__::LEN);
				let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
						&[solana_program::system_instruction::create_account(
								&payer.pubkey(),
								&account.pubkey(),
								account_rent,
								snippets::state::__Account__::LEN as u64,
								&snippets::id(),
						)],
						Some(&payer.pubkey()),
						&[&payer, &account],
						recent_blockhash,
				);
				banks_client.process_transaction(transaction).await.unwrap();


				let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
						&[snippets::instruction::$instruction_name(&snippets::id(), &account.pubkey()).unwrap()],
						Some(&payer.pubkey()),
						&[&payer],
						recent_blockhash,
				);
				banks_client.process_transaction(transaction).await.unwrap();
    };
}