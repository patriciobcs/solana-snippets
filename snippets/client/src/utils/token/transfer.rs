use crate::utils::token::create_associated_token_account;

use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

pub async fn transfer(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    from: &Keypair,
    to: &Keypair,
    amount: u64,
) -> Result<(), BanksClientError> {
    let to_token_account = create_associated_token_account(context, &to.pubkey(), mint).await?;

    let from_token_account =
        spl_associated_token_account::get_associated_token_address(&from.pubkey(), mint);

    let tx = Transaction::new_signed_with_payer(
        &[spl_token::instruction::transfer(
            &spl_token::id(),
            &from_token_account,
            &to_token_account,
            &from.pubkey(),
            &[&from.pubkey()],
            amount,
        )
        .unwrap()],
        Some(&from.pubkey()),
        &[from],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}