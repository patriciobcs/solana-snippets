#![cfg(feature = "test-sbf")]

use scaffold::{instruction, processor::Processor, id, state::Account};
use solana_sdk::{transaction::Transaction, signature::{Signer, Keypair}, program_pack::Pack, system_instruction::create_account};
use solana_program_test::{processor, tokio, ProgramTest};

#[tokio::test]
async fn generic_instruction() {
    let pt = ProgramTest::new("scaffold", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = pt.start().await;


    let account = Keypair::new();

    let rent = banks_client.get_rent().await.unwrap();
    let account_rent = rent.minimum_balance(Account::LEN);
    let transaction = Transaction::new_signed_with_payer(
        &[create_account(
            &payer.pubkey(),
            &account.pubkey(),
            account_rent,
            Account::LEN as u64,
            &id(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &account],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[instruction::generic_instruction(&id(), &account.pubkey()).unwrap()],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();
}
