//* title: Get Funded Keypair
//* description: Gets the keypair with funds
//* platform: client
//* category: system
//* prefix: gfkeypair
//* requires
use solana_program::system_instruction;
use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

pub async fn get_funded_keypair(
	context: &mut ProgramTestContext,
) -> Result<Keypair, BanksClientError> {
  /*/* content */*/
  let __keypair__ = Keypair::new();
  let amount = 1_000_000_000;

  let tx = Transaction::new_signed_with_payer(
      &[system_instruction::transfer(
          &context.payer.pubkey(),
          &__keypair__.pubkey(),
          amount,
      )],
      Some(&context.payer.pubkey()),
      &[&context.payer],
      context.last_blockhash,
  );

  context.banks_client.process_transaction(tx).await.unwrap();
	/*/* content */*/
	Ok(__keypair__)
}