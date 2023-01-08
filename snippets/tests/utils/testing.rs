#![cfg(feature = "test-sbf")]

#[macro_export]
macro_rules! get_program_context {
    ($program:ident) => {{
        let program = solana_program_test::ProgramTest::new(
            stringify!($program),
            $program::id(),
            solana_program_test::processor!($program::processor::Processor::process),
        );
        program.start_with_context().await
    }};
}

#[macro_export]
macro_rules! call_instruction {
    ($context:ident, $instruction:expr; signed by $signers:expr) => {
        let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
            &[$instruction],
            Some(&$context.payer.pubkey()),
            &$signers,
            $context.last_blockhash,
        );
        $context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap();
    };
}

#[macro_export]
macro_rules! test_instruction {
    (no_accounts, $instruction_name:ident) => {
        #[tokio::test]
        async fn $instruction_name() {
            let mut context = get_program_context!(snippets);
            call_instruction!(context, snippets::instruction::$instruction_name(&snippets::id()).unwrap(); signed by [&context.payer]);
        }
    };
    (single_readonly_account, $instruction_name:ident) => {
        #[tokio::test]
        async fn $instruction_name() {
            let mut context = get_program_context!(snippets);

            let account = solana_sdk::signature::Keypair::new();
            let rent = context.banks_client.get_rent().await.unwrap();
            let account_rent = rent.minimum_balance(snippets::state::__Account__::LEN);

            call_instruction!(context, solana_program::system_instruction::create_account(
                &context.payer.pubkey(), 
                &account.pubkey(), 
                account_rent, 
                snippets::state::__Account__::LEN as u64, 
                &snippets::id()); 
                signed by [&context.payer, &account]
            );

            call_instruction!(context, snippets::instruction::$instruction_name(
                &snippets::id(), 
                &account.pubkey()).unwrap();
                signed by [&context.payer]
            );
        }
    };
    (transfer_sol) => {
        #[tokio::test]
        async fn transfer_sol() {
            let mut context = get_program_context!(snippets);

            let receiver = solana_sdk::signature::Keypair::new();
            let payer_key = context.payer.pubkey();

            utils::system::airdrop(&mut context, &payer_key, 10000000).await.unwrap();
            utils::system::airdrop(&mut context, &receiver.pubkey(), 10000000).await.unwrap();

            call_instruction!(context, snippets::instruction::transfer_sol(
                &snippets::id(), 
                &payer_key, 
                &receiver.pubkey(), 
                &solana_program::system_program::id(), 
                100000).unwrap();
                signed by [&context.payer]
            );
        }
    };
}
