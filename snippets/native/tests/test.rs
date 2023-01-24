//* title: Native Test Instruction
//* description: Test any instruction of your Solana program
//* platform: anchor
//* category: test
//* display: vscode

/*/* content */*/
#![cfg(feature = "test-sbf")]

use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction::new_signed_with_payer;
use solana_sdk::instruction::Instruction;
use solana_program_test::*;

#[tokio::test]
async fn __test_name__() {
		let program = ProgramTest::new(
				"__native_program__",
				__native_program__::id(),
				solana_program_test::processor!(__native_program__::processor::Processor::process),
		);
    let mut context = program.start_with_context().await;

		let instruction = __native_program__::instruction:::__instruction_name__(
			&__native_program__::id(),
		).unwrap()

		let transaction = new_signed_with_payer(
				&[instruction],
				Some(context.payer.pubkey()),
				&[&context.payer],
				$context.last_blockhash,
		);
		
		context.banks_client.process_transaction(transaction).await.unwrap();
}
/*/* content */*/