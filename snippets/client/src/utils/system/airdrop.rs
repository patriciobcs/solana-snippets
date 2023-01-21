//* title: Airdrop
//* description: Airdrop an account
//* platform: client
//* category: system
//* prefix: airdrop
//* requires
use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

pub async fn airdrop(
  context: &mut ProgramTestContext,
  __receiver__: &Pubkey,
  __amount__: u64,
) -> Result<(), BanksClientError> {
    /*/* content */*/
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &context.payer.pubkey(),
            __receiver__,
            __amount__,
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
    /*/* content */*/
    Ok(())
}