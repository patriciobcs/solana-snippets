//* title: Transfer Lamports
//* description: Transfer lamports from one account to another
//* platform: client
//* category: system
//* requires
use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

pub async fn transfer_lamports(
    context: &mut ProgramTestContext,
    __from__: &Keypair,
    __to__: &Pubkey,
    __amount__: u64,
) -> Result<(), BanksClientError> {
    /*/* content */*/
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&__from__.pubkey(), __to__, __amount__)],
        Some(&__from__.pubkey()),
        &[__from__],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await?;
    /*/* content */*/   
    Ok(())
}