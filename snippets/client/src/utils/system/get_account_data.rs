//* title: Get Account Data
//* description: Gets the data of an account
//* platform: client
//* category: system
//* prefix: gaccount
//* requires
use solana_program_test::ProgramTestContext;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::account::Account;

pub async fn get_account_data(context: &mut ProgramTestContext, __pubkey__: &Pubkey) -> Account {
    /*/* content */*/
    let __account_data__ = context
        .banks_client
        .get_account(*__pubkey__)
        .await
        .expect("account not found")
        .expect("account empty");
    /*/* content */*/
    __account_data__
}