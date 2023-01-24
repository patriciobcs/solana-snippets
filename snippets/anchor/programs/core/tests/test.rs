//* title: Anchor Test Instruction
//* description: Test any instruction of your Anchor program
//* platform: anchor
//* category: test
//* display: vscode

/*/* content */*/
#![cfg(feature = "test-sbf")]

use anchor_lang::{InstructionData, ToAccountMetas};
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::instruction::Instruction;
use solana_program_test::*;

#[tokio::test]
async fn __test_name__() {
    let program = ProgramTest::new(
        "__anchor_program__",
        __anchor_program__::id(),
        solana_program_test::processor!(__anchor_program__::entry),
    );

    let mut context = program.start_with_context().await;

    let instruction = Instruction {
      program_id: __anchor_program__::id(),
      accounts: __anchor_program__::accounts::__InstructionName__ {

      }.to_account_metas(None),
      data: __anchor_program__::instruction::__InstructionName__ {}.data(),
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    
    context.banks_client.process_transaction(transaction).await.unwrap();
}
/*/* content */*/