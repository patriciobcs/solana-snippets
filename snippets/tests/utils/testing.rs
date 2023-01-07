#![cfg(feature = "test-sbf")]

#[macro_export]
macro_rules! test_instruction {
    (no_accounts, $instruction_name:ident) => {
        #[tokio::test]
        async fn $instruction_name() {
            let program = solana_program_test::ProgramTest::new(
                "snippets",
                snippets::id(),
                solana_program_test::processor!(snippets::processor::Processor::process),
            );
            let mut context = program.start_with_context().await;
            let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
                &[snippets::instruction::$instruction_name(&snippets::id()).unwrap()],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            );
            context.banks_client.process_transaction(transaction).await.unwrap();
        }
    };
    (single_readonly_account, $instruction_name:ident) => {
        #[tokio::test]
        async fn $instruction_name() {
            let program = solana_program_test::ProgramTest::new(
                "snippets",
                snippets::id(),
                solana_program_test::processor!(snippets::processor::Processor::process),
            );
            let mut context = program.start_with_context().await;

            let account = solana_sdk::signature::Keypair::new();
            let rent = context.banks_client.get_rent().await.unwrap();
            let account_rent = rent.minimum_balance(snippets::state::__Account__::LEN);
            let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
                &[solana_program::system_instruction::create_account(
                    &context.payer.pubkey(),
                    &account.pubkey(),
                    account_rent,
                    snippets::state::__Account__::LEN as u64,
                    &snippets::id(),
                )],
                Some(&context.payer.pubkey()),
                &[&context.payer, &account],
                context.last_blockhash,
            );
            context.banks_client.process_transaction(transaction).await.unwrap();

            let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
                &[
                    snippets::instruction::$instruction_name(&snippets::id(), &account.pubkey())
                        .unwrap(),
                ],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            );
            context.banks_client.process_transaction(transaction).await.unwrap();
        }
    };
    (transfer_sol) => {
        #[tokio::test]
        async fn transfer_sol() {
            let program = solana_program_test::ProgramTest::new(
                "snippets",
                snippets::id(),
                solana_program_test::processor!(snippets::processor::Processor::process),
            );
            let mut context = program.start_with_context().await;

            let receiver = solana_sdk::signature::Keypair::new();
            let payer_key = context.payer.pubkey();

            utils::system::airdrop(&mut context, &payer_key, 10000000).await.unwrap();
            utils::system::airdrop(&mut context, &receiver.pubkey(), 10000000).await.unwrap();

            let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
                &[
                    snippets::instruction::transfer_sol(&snippets::id(), &context.payer.pubkey(), &receiver.pubkey(), &solana_program::system_program::id(), 100000)
                        .unwrap(),
                ],
                Some(&context.payer.pubkey()),
                &[&context.payer],
                context.last_blockhash,
            );
            context.banks_client.process_transaction(transaction).await.unwrap();

        }
    };
}
