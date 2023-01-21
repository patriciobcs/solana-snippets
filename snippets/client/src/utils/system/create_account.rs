//* title: Create Account
//* description: Creates an account of size len
//* platform: client
//* category: system
//* prefix: caccount
//* requires
use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

pub async fn create_account(
  context: &mut ProgramTestContext,
  __account__: &Keypair,
  __owner__: &Pubkey,
  __len__: usize,
) -> Result<(), BanksClientError> {
    /*/* content */*/
    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(__len__);

    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::create_account(
            &context.payer.pubkey(),
            &__account__.pubkey(),
            lamports,
            __len__ as u64,
            __owner__,
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &__account__],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
    /*/* content */*/
    Ok(())
}