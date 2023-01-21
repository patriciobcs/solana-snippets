use crate::utils::system::get_account_data;

use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::program_pack::Pack;
use spl_token::state::Mint;

pub async fn get_mint(context: &mut ProgramTestContext, pubkey: &Pubkey) -> Mint {
    let account = get_account_data(context, pubkey).await;
    Mint::unpack(&account.data).unwrap()
}