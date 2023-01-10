#![cfg(feature = "test-sbf")]

use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::program_pack::Pack;
use spl_token::state::Mint;

pub async fn get_mint(context: &mut ProgramTestContext, pubkey: &Pubkey) -> Mint {
    let account = crate::utils::system::get_account_data(context, pubkey).await;
    Mint::unpack(&account.data).unwrap()
}

pub async fn mint_tokens(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    account: &Pubkey,
    amount: u64,
    owner: &Pubkey,
    additional_signer: Option<&Keypair>,
) -> Result<(), BanksClientError> {
    let mut signing_keypairs = vec![&context.payer];
    if let Some(signer) = additional_signer {
        signing_keypairs.push(signer);
    }

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_token::instruction::mint_to(&spl_token::id(), mint, account, owner, &[], amount)
                .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &signing_keypairs,
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn create_token_account(
    context: &mut ProgramTestContext,
    account: &Keypair,
    mint: &Pubkey,
    manager: &Pubkey,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &account.pubkey(),
                rent.minimum_balance(spl_token::state::Account::LEN),
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &account.pubkey(),
                mint,
                manager,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, account],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn get_token_account(
    client: &mut ProgramTestContext,
    token_account: &Pubkey,
) -> Result<spl_token::state::Account, BanksClientError> {
    let account = client.banks_client.get_account(*token_account).await?;
    Ok(spl_token::state::Account::unpack(&account.unwrap().data).unwrap())
}


pub async fn create_associated_token_account(
    context: &mut ProgramTestContext,
    wallet: &Pubkey,
    token_mint: &Pubkey,
) -> Result<Pubkey, BanksClientError> {
    let recent_blockhash = context.last_blockhash;

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_associated_token_account::instruction::create_associated_token_account(
                &context.payer.pubkey(),
                &wallet,
                token_mint,
                &spl_token::id(),
            ),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        recent_blockhash,
    );

    // connection.send_and_confirm_transaction(&tx)?;
    context.banks_client.process_transaction(tx).await.unwrap();

    Ok(spl_associated_token_account::get_associated_token_address(
        &wallet,
        token_mint,
    ))
}

pub async fn create_mint(
    context: &mut ProgramTestContext,
    mint: &Keypair,
    authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &mint.pubkey(),
                rent.minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                authority,
                freeze_authority,
                0,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, mint],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

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

pub async fn create_mint_token_pair(
    context: &mut ProgramTestContext,
    amount: u64,
) -> Result<(Keypair, Pubkey), BanksClientError> {
    let mint = Keypair::new();
	let authority = context.payer.pubkey(); 

	create_mint(context, &mint, &authority, Some(&authority)).await.unwrap();
    
	let ata = create_associated_token_account(context, &authority, &mint.pubkey()).await.unwrap();

    if amount > 0 {
        mint_tokens(context, &mint.pubkey(), &ata, amount, &authority, None).await.unwrap();
    }

    Ok((mint, ata))
}