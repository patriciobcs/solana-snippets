#![cfg(feature = "test-sbf")]

use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::account::Account;

pub async fn transfer_lamports(
    client: &mut ProgramTestContext,
    wallet: &Keypair,
    to: &Pubkey,
    amount: u64,
) -> Result<(), BanksClientError> {
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(&wallet.pubkey(), to, amount)],
        Some(&wallet.pubkey()),
        &[wallet],
        client.last_blockhash,
    );

    client.banks_client.process_transaction(tx).await?;

    Ok(())
}

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

pub async fn get_funded_keypair(
    context: &mut ProgramTestContext,
) -> Result<Keypair, BanksClientError> {
    let keypair = Keypair::new();
    airdrop(context, &keypair.pubkey(), 1_000_000_000)
        .await
        .unwrap();
    Ok(keypair)
}

pub fn clone_keypair(keypair: &Keypair) -> Keypair {
    Keypair::from_bytes(&keypair.to_bytes()).unwrap()
}

pub async fn get_account_data(context: &mut ProgramTestContext, pubkey: &Pubkey) -> Account {
    context
        .banks_client
        .get_account(*pubkey)
        .await
        .expect("account not found")
        .expect("account empty")
}

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