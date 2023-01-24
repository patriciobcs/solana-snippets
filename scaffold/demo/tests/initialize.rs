#![cfg(feature = "test-sbf")]

use anchor_lang::{InstructionData, ToAccountMetas};
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::instruction::Instruction;
use solana_program_test::*;

#[tokio::test]
async fn test_initialize() {
let program = ProgramTest::new(
    "demo",
    demo::id(),
    solana_program_test::processor!(demo::entry),
);

let mut context = program.start_with_context().await;

let instruction = Instruction {
  program_id: demo::id(),
  accounts: demo::accounts::Initialize {
    user: context.payer.pubkey(),
    demo: solana_sdk::pubkey::Pubkey::find_program_address(&[b"demo"], &demo::id()).0,
    system_program: solana_program::system_program::id(),
  }.to_account_metas(None),
  data: demo::instruction::Initialize {}.data(),
};

let transaction = Transaction::new_signed_with_payer(
    &[instruction],
    Some(&context.payer.pubkey()),
    &[&context.payer],
    context.last_blockhash,
);

context.banks_client.process_transaction(transaction).await.unwrap();
}