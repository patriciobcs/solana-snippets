use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

pub async fn airdrop(
	context: &mut ProgramTestContext,
	receiver: &Pubkey,
	amount: u64,
) -> Result<(), BanksClientError> {
	let tx = Transaction::new_signed_with_payer(
			&[system_instruction::transfer(
					&context.payer.pubkey(),
					receiver,
					amount,
			)],
			Some(&context.payer.pubkey()),
			&[&context.payer],
			context.last_blockhash,
	);

	context.banks_client.process_transaction(tx).await.unwrap();
	Ok(())
}
