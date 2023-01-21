use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

pub async fn create_account(
	context: &mut ProgramTestContext,
	account: &Keypair,
	owner: &Pubkey,
	len: usize,
) -> Result<(), BanksClientError> {
	let rent = context.banks_client.get_rent().await.unwrap();
	let lamports = rent.minimum_balance(len);

	let tx = Transaction::new_signed_with_payer(
			&[system_instruction::create_account(
					&context.payer.pubkey(),
					&account.pubkey(),
					lamports,
					len as u64,
					owner,
			)],
			Some(&context.payer.pubkey()),
			&[&context.payer, &account],
			context.last_blockhash,
	);

	context.banks_client.process_transaction(tx).await.unwrap();
	Ok(())
}