#![cfg(feature = "test-sbf")]

use snippets::{instruction, processor::Processor, id};
use solana_sdk::{transaction::Transaction, signature::Signer};
use solana_program_test::{processor, tokio, ProgramTest};

#[tokio::test]
async fn grent() {
    let pt = ProgramTest::new("snippets", id(), processor!(Processor::process));
    let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction::grent(&id()).unwrap()],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();
}
