use crate::utils::token::create_mint;
use crate::utils::token::create_associated_token_account;
use crate::utils::token::mint_tokens;

use solana_program_test::ProgramTestContext;
use solana_program_test::BanksClientError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

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