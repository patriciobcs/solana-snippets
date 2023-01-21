use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::account::Account;

pub async fn get_account_data(context: &mut ProgramTestContext, pubkey: &Pubkey) -> Account {
	context
			.banks_client
			.get_account(*pubkey)
			.await
			.expect("account not found")
			.expect("account empty")
}