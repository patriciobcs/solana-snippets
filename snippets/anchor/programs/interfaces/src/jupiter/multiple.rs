use anchor_lang::prelude::*;

declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

//* title: Jupiter's Instruction Route
//* description: Creates the interface instruction `route` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct Route<'info> {
  pub token_program: AccountInfo<'info>,
  #[account(signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  pub destination_token_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction WhirlpoolSwapExactOutput
//* description: Creates the interface instruction `whirlpoolSwapExactOutput` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct WhirlpoolSwapExactOutput<'info> {
  pub swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  #[account(signer)]
  pub token_authority: AccountInfo<'info>,
  #[account(mut)]
  pub whirlpool: AccountInfo<'info>,
  #[account(mut)]
  pub token_owner_account_a: AccountInfo<'info>,
  #[account(mut)]
  pub token_vault_a: AccountInfo<'info>,
  #[account(mut)]
  pub token_owner_account_b: AccountInfo<'info>,
  #[account(mut)]
  pub token_vault_b: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array0: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array1: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array2: AccountInfo<'info>,
  pub oracle: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction CreateOpenOrders
//* description: Creates the interface instruction `createOpenOrders` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct CreateOpenOrders<'info> {
  #[account(mut)]
  pub open_orders: AccountInfo<'info>,
  #[account(mut, signer)]
  pub payer: AccountInfo<'info>,
  pub dex_program: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
  pub market: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction MercurialSwap
//* description: Creates the interface instruction `mercurialSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct MercurialSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub swap_state: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub pool_authority: AccountInfo<'info>,
  #[account(signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  #[account(mut)]
  pub source_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub destination_token_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction CykuraSwap
//* description: Creates the interface instruction `cykuraSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct CykuraSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  #[account(signer)]
  pub signer: AccountInfo<'info>,
  pub factory_state: AccountInfo<'info>,
  #[account(mut)]
  pub pool_state: AccountInfo<'info>,
  #[account(mut)]
  pub input_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub output_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub input_vault: AccountInfo<'info>,
  #[account(mut)]
  pub output_vault: AccountInfo<'info>,
  #[account(mut)]
  pub last_observation_state: AccountInfo<'info>,
  pub core_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction SerumSwap
//* description: Creates the interface instruction `serumSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct MarketSerumSwap<'info> {
  #[account(mut)]
  pub market: AccountInfo<'info>,
  #[account(mut)]
  pub open_orders: AccountInfo<'info>,
  #[account(mut)]
  pub request_queue: AccountInfo<'info>,
  #[account(mut)]
  pub event_queue: AccountInfo<'info>,
  #[account(mut)]
  pub bids: AccountInfo<'info>,
  #[account(mut)]
  pub asks: AccountInfo<'info>,
  #[account(mut)]
  pub coin_vault: AccountInfo<'info>,
  #[account(mut)]
  pub pc_vault: AccountInfo<'info>,
  pub vault_signer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SerumSwap<'info> {
  pub market_serum_swap: MarketSerumSwap<'info>,
  #[account(signer)]
  pub authority: AccountInfo<'info>,
  #[account(mut)]
  pub order_payer_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub coin_wallet: AccountInfo<'info>,
  #[account(mut)]
  pub pc_wallet: AccountInfo<'info>,
  pub dex_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction SaberSwap
//* description: Creates the interface instruction `saberSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct SaberSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub swap: AccountInfo<'info>,
  pub swap_authority: AccountInfo<'info>,
  pub user_authority: AccountInfo<'info>,
  #[account(mut)]
  pub input_user_account: AccountInfo<'info>,
  #[account(mut)]
  pub input_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub output_user_account: AccountInfo<'info>,
  #[account(mut)]
  pub output_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub fees_token_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction SaberAddDecimals
//* description: Creates the interface instruction `saberAddDecimals` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct SaberAddDecimals<'info> {
  pub add_decimals_program: AccountInfo<'info>,
  pub wrapper: AccountInfo<'info>,
  #[account(mut)]
  pub wrapper_mint: AccountInfo<'info>,
  #[account(mut)]
  pub wrapper_underlying_tokens: AccountInfo<'info>,
  #[account(signer)]
  pub owner: AccountInfo<'info>,
  #[account(mut)]
  pub user_underlying_tokens: AccountInfo<'info>,
  #[account(mut)]
  pub user_wrapped_tokens: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction TokenSwap
//* description: Creates the interface instruction `tokenSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct TokenSwap<'info> {
  pub token_swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub swap: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
  #[account(signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  #[account(mut)]
  pub source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_destination: AccountInfo<'info>,
  #[account(mut)]
  pub destination: AccountInfo<'info>,
  #[account(mut)]
  pub pool_mint: AccountInfo<'info>,
  #[account(mut)]
  pub pool_fee: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction SenchaSwap
//* description: Creates the interface instruction `senchaSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct SenchaSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  #[account(mut)]
  pub swap: AccountInfo<'info>,
  pub user_authority: AccountInfo<'info>,
  #[account(mut)]
  pub input_user_account: AccountInfo<'info>,
  #[account(mut)]
  pub input_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub input_fees_account: AccountInfo<'info>,
  #[account(mut)]
  pub output_user_account: AccountInfo<'info>,
  #[account(mut)]
  pub output_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub output_fees_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction StepSwap
//* description: Creates the interface instruction `stepSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct StepSwap<'info> {
  pub token_swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub swap: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
  #[account(signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  #[account(mut)]
  pub source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_destination: AccountInfo<'info>,
  #[account(mut)]
  pub destination: AccountInfo<'info>,
  #[account(mut)]
  pub pool_mint: AccountInfo<'info>,
  #[account(mut)]
  pub pool_fee: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction CropperSwap
//* description: Creates the interface instruction `cropperSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct CropperSwap<'info> {
  pub token_swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub swap: AccountInfo<'info>,
  pub swap_state: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
  #[account(signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  #[account(mut)]
  pub source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_destination: AccountInfo<'info>,
  #[account(mut)]
  pub destination: AccountInfo<'info>,
  #[account(mut)]
  pub pool_mint: AccountInfo<'info>,
  #[account(mut)]
  pub pool_fee: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction RaydiumSwap
//* description: Creates the interface instruction `raydiumSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  #[account(mut)]
  pub amm_id: AccountInfo<'info>,
  pub amm_authority: AccountInfo<'info>,
  #[account(mut)]
  pub amm_open_orders: AccountInfo<'info>,
  #[account(mut)]
  pub pool_coin_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub pool_pc_token_account: AccountInfo<'info>,
  pub serum_program_id: AccountInfo<'info>,
  #[account(mut)]
  pub serum_market: AccountInfo<'info>,
  #[account(mut)]
  pub serum_bids: AccountInfo<'info>,
  #[account(mut)]
  pub serum_asks: AccountInfo<'info>,
  #[account(mut)]
  pub serum_event_queue: AccountInfo<'info>,
  #[account(mut)]
  pub serum_coin_vault_account: AccountInfo<'info>,
  #[account(mut)]
  pub serum_pc_vault_account: AccountInfo<'info>,
  pub serum_vault_signer: AccountInfo<'info>,
  #[account(mut)]
  pub user_source_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub user_destination_token_account: AccountInfo<'info>,
  #[account(signer)]
  pub user_source_owner: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction CremaSwap
//* description: Creates the interface instruction `cremaSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct CremaSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub clmm_config: AccountInfo<'info>,
  #[account(mut)]
  pub clmmpool: AccountInfo<'info>,
  pub token_a: AccountInfo<'info>,
  pub token_b: AccountInfo<'info>,
  #[account(mut)]
  pub account_a: AccountInfo<'info>,
  #[account(mut)]
  pub account_b: AccountInfo<'info>,
  #[account(mut)]
  pub token_a_vault: AccountInfo<'info>,
  #[account(mut)]
  pub token_b_vault: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array_map: AccountInfo<'info>,
  #[account(signer)]
  pub owner: AccountInfo<'info>,
  pub partner: AccountInfo<'info>,
  #[account(mut)]
  pub partner_ata_a: AccountInfo<'info>,
  #[account(mut)]
  pub partner_ata_b: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction LifinitySwap
//* description: Creates the interface instruction `lifinitySwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct LifinitySwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
  pub amm: AccountInfo<'info>,
  #[account(signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  #[account(mut)]
  pub source_info: AccountInfo<'info>,
  #[account(mut)]
  pub destination_info: AccountInfo<'info>,
  #[account(mut)]
  pub swap_source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_destination: AccountInfo<'info>,
  #[account(mut)]
  pub pool_mint: AccountInfo<'info>,
  #[account(mut)]
  pub fee_account: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub pyth_account: AccountInfo<'info>,
  pub pyth_pc_account: AccountInfo<'info>,
  #[account(mut)]
  pub config_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction MarinadeDeposit
//* description: Creates the interface instruction `marinadeDeposit` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct MarinadeDeposit<'info> {
  pub marinade_finance_program: AccountInfo<'info>,
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub msol_mint: AccountInfo<'info>,
  #[account(mut)]
  pub liq_pool_sol_leg_pda: AccountInfo<'info>,
  #[account(mut)]
  pub liq_pool_msol_leg: AccountInfo<'info>,
  pub liq_pool_msol_leg_authority: AccountInfo<'info>,
  #[account(mut)]
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut)]
  pub transfer_from: AccountInfo<'info>,
  #[account(mut)]
  pub mint_to: AccountInfo<'info>,
  pub msol_mint_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  #[account(mut)]
  pub user_wsol_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub temp_wsol_token_account: AccountInfo<'info>,
  #[account(mut, signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  pub wsol_mint: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction MarinadeUnstake
//* description: Creates the interface instruction `marinadeUnstake` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct MarinadeUnstake<'info> {
  pub marinade_finance_program: AccountInfo<'info>,
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub msol_mint: AccountInfo<'info>,
  #[account(mut)]
  pub liq_pool_sol_leg_pda: AccountInfo<'info>,
  #[account(mut)]
  pub liq_pool_msol_leg: AccountInfo<'info>,
  #[account(mut)]
  pub treasury_msol_account: AccountInfo<'info>,
  #[account(mut)]
  pub get_msol_from: AccountInfo<'info>,
  #[account(signer)]
  pub get_msol_from_authority: AccountInfo<'info>,
  #[account(mut)]
  pub transfer_sol_to: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  #[account(mut)]
  pub user_wsol_token_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction AldrinSwap
//* description: Creates the interface instruction `aldrinSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct AldrinSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub pool: AccountInfo<'info>,
  pub pool_signer: AccountInfo<'info>,
  #[account(mut)]
  pub pool_mint: AccountInfo<'info>,
  #[account(mut)]
  pub base_token_vault: AccountInfo<'info>,
  #[account(mut)]
  pub quote_token_vault: AccountInfo<'info>,
  #[account(mut)]
  pub fee_pool_token_account: AccountInfo<'info>,
  #[account(signer)]
  pub wallet_authority: AccountInfo<'info>,
  #[account(mut)]
  pub user_base_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub user_quote_token_account: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction AldrinV2Swap
//* description: Creates the interface instruction `aldrinV2Swap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct AldrinV2Swap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub pool: AccountInfo<'info>,
  pub pool_signer: AccountInfo<'info>,
  #[account(mut)]
  pub pool_mint: AccountInfo<'info>,
  #[account(mut)]
  pub base_token_vault: AccountInfo<'info>,
  #[account(mut)]
  pub quote_token_vault: AccountInfo<'info>,
  #[account(mut)]
  pub fee_pool_token_account: AccountInfo<'info>,
  #[account(signer)]
  pub wallet_authority: AccountInfo<'info>,
  #[account(mut)]
  pub user_base_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub user_quote_token_account: AccountInfo<'info>,
  pub curve: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction WhirlpoolSwap
//* description: Creates the interface instruction `whirlpoolSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct WhirlpoolSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  #[account(signer)]
  pub token_authority: AccountInfo<'info>,
  #[account(mut)]
  pub whirlpool: AccountInfo<'info>,
  #[account(mut)]
  pub token_owner_account_a: AccountInfo<'info>,
  #[account(mut)]
  pub token_vault_a: AccountInfo<'info>,
  #[account(mut)]
  pub token_owner_account_b: AccountInfo<'info>,
  #[account(mut)]
  pub token_vault_b: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array0: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array1: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array2: AccountInfo<'info>,
  pub oracle: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction InvariantSwap
//* description: Creates the interface instruction `invariantSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct InvariantSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountInfo<'info>,
  #[account(mut)]
  pub tickmap: AccountInfo<'info>,
  #[account(mut)]
  pub account_x: AccountInfo<'info>,
  #[account(mut)]
  pub account_y: AccountInfo<'info>,
  #[account(mut)]
  pub reserve_x: AccountInfo<'info>,
  #[account(mut)]
  pub reserve_y: AccountInfo<'info>,
  #[account(signer)]
  pub owner: AccountInfo<'info>,
  pub program_authority: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction MeteoraSwap
//* description: Creates the interface instruction `meteoraSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct MeteoraSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountInfo<'info>,
  #[account(mut)]
  pub user_source_token: AccountInfo<'info>,
  #[account(mut)]
  pub user_destination_token: AccountInfo<'info>,
  #[account(mut)]
  pub a_vault: AccountInfo<'info>,
  #[account(mut)]
  pub b_vault: AccountInfo<'info>,
  #[account(mut)]
  pub a_token_vault: AccountInfo<'info>,
  #[account(mut)]
  pub b_token_vault: AccountInfo<'info>,
  #[account(mut)]
  pub a_vault_lp_mint: AccountInfo<'info>,
  #[account(mut)]
  pub b_vault_lp_mint: AccountInfo<'info>,
  #[account(mut)]
  pub a_vault_lp: AccountInfo<'info>,
  #[account(mut)]
  pub b_vault_lp: AccountInfo<'info>,
  #[account(mut)]
  pub admin_token_fee: AccountInfo<'info>,
  #[account(signer)]
  pub user: AccountInfo<'info>,
  pub vault_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction GoosefxSwap
//* description: Creates the interface instruction `goosefxSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct GoosefxSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub controller: AccountInfo<'info>,
  #[account(mut)]
  pub pair: AccountInfo<'info>,
  #[account(mut)]
  pub ssl_in: AccountInfo<'info>,
  #[account(mut)]
  pub ssl_out: AccountInfo<'info>,
  #[account(mut)]
  pub liability_vault_in: AccountInfo<'info>,
  #[account(mut)]
  pub swapped_liability_vault_in: AccountInfo<'info>,
  #[account(mut)]
  pub liability_vault_out: AccountInfo<'info>,
  #[account(mut)]
  pub swapped_liability_vault_out: AccountInfo<'info>,
  #[account(mut)]
  pub user_in_ata: AccountInfo<'info>,
  #[account(mut)]
  pub user_out_ata: AccountInfo<'info>,
  #[account(mut)]
  pub fee_collector_ata: AccountInfo<'info>,
  #[account(signer)]
  pub user_wallet: AccountInfo<'info>,
  pub fee_collector: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction DeltafiSwap
//* description: Creates the interface instruction `deltafiSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct DeltafiSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub market_config: AccountInfo<'info>,
  #[account(mut)]
  pub swap_info: AccountInfo<'info>,
  #[account(mut)]
  pub user_source_token: AccountInfo<'info>,
  #[account(mut)]
  pub user_destination_token: AccountInfo<'info>,
  #[account(mut)]
  pub swap_source_token: AccountInfo<'info>,
  #[account(mut)]
  pub swap_destination_token: AccountInfo<'info>,
  #[account(mut)]
  pub deltafi_user: AccountInfo<'info>,
  #[account(mut)]
  pub admin_destination_token: AccountInfo<'info>,
  pub pyth_price_base: AccountInfo<'info>,
  pub pyth_price_quote: AccountInfo<'info>,
  #[account(signer)]
  pub user_authority: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction BalansolSwap
//* description: Creates the interface instruction `balansolSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct BalansolSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  #[account(mut, signer)]
  pub authority: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountInfo<'info>,
  #[account(mut)]
  pub tax_man: AccountInfo<'info>,
  pub bid_mint: AccountInfo<'info>,
  pub treasurer: AccountInfo<'info>,
  #[account(mut)]
  pub src_treasury: AccountInfo<'info>,
  #[account(mut)]
  pub src_associated_token_account: AccountInfo<'info>,
  pub ask_mint: AccountInfo<'info>,
  #[account(mut)]
  pub dst_treasury: AccountInfo<'info>,
  #[account(mut)]
  pub dst_associated_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub dst_token_account_taxman: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub associated_token_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction MarcoPoloSwap
//* description: Creates the interface instruction `marcoPoloSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct MarcoPoloSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub pool: AccountInfo<'info>,
  pub token_x: AccountInfo<'info>,
  pub token_y: AccountInfo<'info>,
  #[account(mut)]
  pub pool_x_account: AccountInfo<'info>,
  #[account(mut)]
  pub pool_y_account: AccountInfo<'info>,
  #[account(mut)]
  pub swapper_x_account: AccountInfo<'info>,
  #[account(mut)]
  pub swapper_y_account: AccountInfo<'info>,
  #[account(mut, signer)]
  pub swapper: AccountInfo<'info>,
  #[account(mut)]
  pub referrer_x_account: AccountInfo<'info>,
  #[account(mut)]
  pub referrer_y_account: AccountInfo<'info>,
  #[account(mut)]
  pub referrer: AccountInfo<'info>,
  pub program_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub associated_token_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction DradexSwap
//* description: Creates the interface instruction `dradexSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct DradexSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  #[account(mut)]
  pub pair: AccountInfo<'info>,
  #[account(mut)]
  pub market: AccountInfo<'info>,
  #[account(mut)]
  pub event_queue: AccountInfo<'info>,
  pub dex_user: AccountInfo<'info>,
  #[account(mut)]
  pub market_user: AccountInfo<'info>,
  #[account(mut)]
  pub bids: AccountInfo<'info>,
  #[account(mut)]
  pub asks: AccountInfo<'info>,
  #[account(mut)]
  pub t0_vault: AccountInfo<'info>,
  #[account(mut)]
  pub t1_vault: AccountInfo<'info>,
  #[account(mut)]
  pub t0_user: AccountInfo<'info>,
  #[account(mut)]
  pub t1_user: AccountInfo<'info>,
  pub master: AccountInfo<'info>,
  #[account(mut, signer)]
  pub signer: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub logger: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction LifinityV2Swap
//* description: Creates the interface instruction `lifinityV2Swap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct LifinityV2Swap<'info> {
  pub swap_program: AccountInfo<'info>,
  pub authority: AccountInfo<'info>,
  #[account(mut)]
  pub amm: AccountInfo<'info>,
  #[account(signer)]
  pub user_transfer_authority: AccountInfo<'info>,
  #[account(mut)]
  pub source_info: AccountInfo<'info>,
  #[account(mut)]
  pub destination_info: AccountInfo<'info>,
  #[account(mut)]
  pub swap_source: AccountInfo<'info>,
  #[account(mut)]
  pub swap_destination: AccountInfo<'info>,
  #[account(mut)]
  pub pool_mint: AccountInfo<'info>,
  #[account(mut)]
  pub fee_account: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub oracle_main_account: AccountInfo<'info>,
  pub oracle_sub_account: AccountInfo<'info>,
  pub oracle_pc_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Instruction RaydiumClmmSwap
//* description: Creates the interface instruction `raydiumClmmSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct RaydiumClmmSwap<'info> {
  pub swap_program: AccountInfo<'info>,
  #[account(signer)]
  pub payer: AccountInfo<'info>,
  pub amm_config: AccountInfo<'info>,
  #[account(mut)]
  pub pool_state: AccountInfo<'info>,
  #[account(mut)]
  pub input_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub output_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub input_vault: AccountInfo<'info>,
  #[account(mut)]
  pub output_vault: AccountInfo<'info>,
  #[account(mut)]
  pub observation_state: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  #[account(mut)]
  pub tick_array: AccountInfo<'info>,
}

/*/* content */*/

//* title: Jupiter's Type AmountWithSlippage
//* description: Generates the type `AmountWithSlippage` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct AmountWithSlippage {
  pub amount: u64,
  pub slippage_bps: u16,
}

/*/* content */*/

//* title: Jupiter's Type SplitLegDeeper
//* description: Generates the type `SplitLegDeeper` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct SplitLegDeeper {
  pub percent: u8,
  pub swap_leg: SwapLegSwap,
}

/*/* content */*/

//* title: Jupiter's Type SplitLeg
//* description: Generates the type `SplitLeg` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct SplitLeg {
  pub percent: u8,
  pub swap_leg: SwapLegDeeper,
}

/*/* content */*/

//* title: Jupiter's Type SwapInstrution
//* description: Generates the type `SwapInstrution` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
/*/* content */*/

//* title: Jupiter's Type Side
//* description: Generates the type `Side` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum Side {
  Bid,
  Ask,
}

/*/* content */*/

//* title: Jupiter's Type SwapLegSwap
//* description: Generates the type `SwapLegSwap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum SwapLegSwap {
  PlaceholderOne,
  PlaceholderTwo,
  Swap { swap: Swap },
}

/*/* content */*/

//* title: Jupiter's Type SwapLegDeeper
//* description: Generates the type `SwapLegDeeper` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum SwapLegDeeper {
  Chain { swap_legs: Vec<SwapLegSwap> },
  Split { split_legs: Vec<SplitLegDeeper> },
  Swap { swap: Swap },
}

/*/* content */*/

//* title: Jupiter's Type SwapLeg
//* description: Generates the type `SwapLeg` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum SwapLeg {
  Chain { swap_legs: Vec<SwapLegDeeper> },
  Split { split_legs: Vec<SplitLeg> },
  Swap { swap: Swap },
}

/*/* content */*/

//* title: Jupiter's Type Swap
//* description: Generates the type `Swap` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum Swap {
  Saber,
  SaberAddDecimalsDeposit,
  SaberAddDecimalsWithdraw,
  TokenSwap,
  Sencha,
  Step,
  Cropper,
  Raydium,
  Crema { a_to_b: bool },
  Lifinity,
  Mercurial,
  Cykura,
  Serum { side: Side },
  MarinadeDeposit,
  MarinadeUnstake,
  Aldrin { side: Side },
  AldrinV2 { side: Side },
  Whirlpool { a_to_b: bool },
  Invariant { x_to_y: bool },
  Meteora,
  GooseFX,
  DeltaFi { stable: bool },
  Balansol,
  MarcoPolo { x_to_y: bool },
  Dradex { side: Side },
  LifinityV2,
  RaydiumClmm,
  Openbook { side: Side },
}

/*/* content */*/

//* title: Jupiter's Type SwapAction
//* description: Generates the type `SwapAction` of the `Jupiter` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum SwapAction {
  SetupSplit { percents: Vec<u8> },
  NextSplitLeg,
  MergeSplit,
  Swap { swap: Swap },
}

/*/* content */*/

