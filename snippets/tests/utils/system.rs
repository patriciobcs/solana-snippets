#![cfg(feature = "test-sbf")]

#[macro_export]
macro_rules! test_instruction {
    (no_accounts, $instruction_name:ident) => {
        #[tokio::test]
        async fn $instruction_name() {
            let pt = solana_program_test::ProgramTest::new(
                "snippets",
                snippets::id(),
                solana_program_test::processor!(snippets::processor::Processor::process),
            );
            let (mut banks_client, payer, recent_blockhash) = pt.start().await;
            let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
                &[snippets::instruction::$instruction_name(&snippets::id()).unwrap()],
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            banks_client.process_transaction(transaction).await.unwrap();
        }
    };
    (single_readonly_account, $instruction_name:ident) => {
        #[tokio::test]
        async fn $instruction_name() {
            let pt = solana_program_test::ProgramTest::new(
                "snippets",
                snippets::id(),
                solana_program_test::processor!(snippets::processor::Processor::process),
            );
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
                &[
                    snippets::instruction::$instruction_name(&snippets::id(), &account.pubkey())
                        .unwrap(),
                ],
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            banks_client.process_transaction(transaction).await.unwrap();
        }
    };
    (transfer_sol) => {
        #[tokio::test]
        async fn transfer_sol() {
            let pt = solana_program_test::ProgramTest::new(
                "snippets",
                snippets::id(),
                solana_program_test::processor!(snippets::processor::Processor::process),
            );
            let (mut banks_client, payer, recent_blockhash) = pt.start().await;

            let receiver = solana_sdk::signature::Keypair::new();

            let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
                &[
                    snippets::instruction::transfer_sol(&snippets::id(), &payer.pubkey(), &receiver.pubkey(), &solana_program::system_program::id(), 100000)
                        .unwrap(),
                ],
                Some(&payer.pubkey()),
                &[&payer],
                recent_blockhash,
            );
            banks_client.process_transaction(transaction).await.unwrap();

        }
    };
}