pub mod utils;

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
macro_rules! test {
    ($instruction_name:ident => $body:block) => {
        #[solana_program_test::tokio::test]
        async fn $instruction_name() {
            $body
        }
    }
}

#[macro_export]
macro_rules! test_instruction {
    (no_accounts, $package:ident::$instruction_name:ident) => {
        test!($instruction_name => {
            let mut context = get_program_context!($package);
            call_instruction!(context, $package::instruction::$instruction_name(&$package::id()).unwrap(); signed by [&context.payer]);
        });
    };
    (single_readonly_account, $package:ident::$instruction_name:ident) => {
        test!($instruction_name => {
            let mut context = get_program_context!($package);

            let account = solana_sdk::signature::Keypair::new();
            let rent = context.banks_client.get_rent().await.unwrap();
            let account_rent = rent.minimum_balance($package::state::__Account__::LEN);

            call_instruction!(context, solana_program::system_instruction::create_account(
                &context.payer.pubkey(), 
                &account.pubkey(), 
                account_rent, 
                $package::state::__Account__::LEN as u64, 
                &$package::id()); 
                signed by [&context.payer, &account]
            );

            call_instruction!(context, $package::instruction::$instruction_name(
                &$package::id(), 
                &account.pubkey()).unwrap();
                signed by [&context.payer]
            );
        });
    };
}

#[macro_export]
macro_rules! get_anchor_program_context {
    ($program:ident) => {{
        let program = solana_program_test::ProgramTest::new(
            stringify!($program),
            $program::id(),
            solana_program_test::processor!($program::entry),
        );
        program.start_with_context().await
    }};
}


#[macro_export]
macro_rules! test_anchor_instruction {
    (no_accounts, $package:ident::$instruction_name:ident::$instruction_symbol:ident) => {
        test!($instruction_name => {
            let mut context = get_anchor_program_context!($package);
            call_instruction!(context, solana_sdk::instruction::Instruction {
                program_id: $package::id(),
                accounts: $package::instructions::$instruction_name::$instruction_symbol {}.to_account_metas(None),
                data: $package::instruction::$instruction_symbol {}.data(),
            }; signed by [&context.payer]);
        });
    };
}