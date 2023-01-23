use anchor_lang::prelude::*;

declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

//* title: Marinade Finance's Instruction Initialize
//* description: Creates the interface instruction `initialize` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct LiqPoolInitialize<'info> {
  pub lp_mint: AccountInfo<'info>,
  pub sol_leg_pda: AccountInfo<'info>,
  pub msol_leg: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(signer)]
  pub creator_authority: AccountInfo<'info>,
  #[account(mut)]
  pub state: AccountInfo<'info>,
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  pub msol_mint: AccountInfo<'info>,
  pub operational_sol_account: AccountInfo<'info>,
  pub liq_pool_initialize: LiqPoolInitialize<'info>,
  pub treasury_msol_account: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction ChangeAuthority
//* description: Creates the interface instruction `changeAuthority` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct ChangeAuthority<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub admin_authority: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction AddValidator
//* description: Creates the interface instruction `addValidator` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct AddValidator<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub manager_authority: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  pub validator_vote: AccountInfo<'info>,
  #[account(mut)]
  pub duplication_flag: AccountInfo<'info>,
  #[account(mut, signer)]
  pub rent_payer: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction RemoveValidator
//* description: Creates the interface instruction `removeValidator` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct RemoveValidator<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub manager_authority: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  #[account(mut)]
  pub duplication_flag: AccountInfo<'info>,
  #[account(mut)]
  pub operational_sol_account: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction SetValidatorScore
//* description: Creates the interface instruction `setValidatorScore` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct SetValidatorScore<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub manager_authority: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction ConfigValidatorSystem
//* description: Creates the interface instruction `configValidatorSystem` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct ConfigValidatorSystem<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub manager_authority: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction Deposit
//* description: Creates the interface instruction `deposit` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct Deposit<'info> {
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
  #[account(mut, signer)]
  pub transfer_from: AccountInfo<'info>,
  #[account(mut)]
  pub mint_to: AccountInfo<'info>,
  pub msol_mint_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction DepositStakeAccount
//* description: Creates the interface instruction `depositStakeAccount` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct DepositStakeAccount<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_account: AccountInfo<'info>,
  #[account(signer)]
  pub stake_authority: AccountInfo<'info>,
  #[account(mut)]
  pub duplication_flag: AccountInfo<'info>,
  #[account(mut, signer)]
  pub rent_payer: AccountInfo<'info>,
  #[account(mut)]
  pub msol_mint: AccountInfo<'info>,
  #[account(mut)]
  pub mint_to: AccountInfo<'info>,
  pub msol_mint_authority: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction LiquidUnstake
//* description: Creates the interface instruction `liquidUnstake` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct LiquidUnstake<'info> {
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
}

/*/* content */*/

//* title: Marinade Finance's Instruction AddLiquidity
//* description: Creates the interface instruction `addLiquidity` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct AddLiquidity<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub lp_mint: AccountInfo<'info>,
  pub lp_mint_authority: AccountInfo<'info>,
  pub liq_pool_msol_leg: AccountInfo<'info>,
  #[account(mut)]
  pub liq_pool_sol_leg_pda: AccountInfo<'info>,
  #[account(mut, signer)]
  pub transfer_from: AccountInfo<'info>,
  #[account(mut)]
  pub mint_to: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction RemoveLiquidity
//* description: Creates the interface instruction `removeLiquidity` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub lp_mint: AccountInfo<'info>,
  #[account(mut)]
  pub burn_from: AccountInfo<'info>,
  #[account(signer)]
  pub burn_from_authority: AccountInfo<'info>,
  #[account(mut)]
  pub transfer_sol_to: AccountInfo<'info>,
  #[account(mut)]
  pub transfer_msol_to: AccountInfo<'info>,
  #[account(mut)]
  pub liq_pool_sol_leg_pda: AccountInfo<'info>,
  #[account(mut)]
  pub liq_pool_msol_leg: AccountInfo<'info>,
  pub liq_pool_msol_leg_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction ConfigLp
//* description: Creates the interface instruction `configLp` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct ConfigLp<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub admin_authority: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction ConfigMarinade
//* description: Creates the interface instruction `configMarinade` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct ConfigMarinade<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub admin_authority: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction OrderUnstake
//* description: Creates the interface instruction `orderUnstake` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct OrderUnstake<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub msol_mint: AccountInfo<'info>,
  #[account(mut)]
  pub burn_msol_from: AccountInfo<'info>,
  #[account(signer)]
  pub burn_msol_authority: AccountInfo<'info>,
  #[account(mut)]
  pub new_ticket_account: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction Claim
//* description: Creates the interface instruction `claim` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct Claim<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut)]
  pub ticket_account: AccountInfo<'info>,
  #[account(mut)]
  pub transfer_sol_to: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction StakeReserve
//* description: Creates the interface instruction `stakeReserve` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct StakeReserve<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub validator_vote: AccountInfo<'info>,
  #[account(mut)]
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut)]
  pub stake_account: AccountInfo<'info>,
  pub stake_deposit_authority: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub epoch_schedule: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
  pub stake_history: AccountInfo<'info>,
  pub stake_config: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction UpdateActive
//* description: Creates the interface instruction `updateActive` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct CommonUpdateActive<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_account: AccountInfo<'info>,
  pub stake_withdraw_authority: AccountInfo<'info>,
  #[account(mut)]
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut)]
  pub msol_mint: AccountInfo<'info>,
  pub msol_mint_authority: AccountInfo<'info>,
  #[account(mut)]
  pub treasury_msol_account: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub stake_history: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateActive<'info> {
  pub common_update_active: CommonUpdateActive<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction UpdateDeactivated
//* description: Creates the interface instruction `updateDeactivated` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct CommonUpdateDeactivated<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_account: AccountInfo<'info>,
  pub stake_withdraw_authority: AccountInfo<'info>,
  #[account(mut)]
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut)]
  pub msol_mint: AccountInfo<'info>,
  pub msol_mint_authority: AccountInfo<'info>,
  #[account(mut)]
  pub treasury_msol_account: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub stake_history: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateDeactivated<'info> {
  pub common_update_deactivated: CommonUpdateDeactivated<'info>,
  #[account(mut)]
  pub operational_sol_account: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction DeactivateStake
//* description: Creates the interface instruction `deactivateStake` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct DeactivateStake<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_account: AccountInfo<'info>,
  pub stake_deposit_authority: AccountInfo<'info>,
  #[account(mut, signer)]
  pub split_stake_account: AccountInfo<'info>,
  #[account(mut, signer)]
  pub split_stake_rent_payer: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
  pub epoch_schedule: AccountInfo<'info>,
  pub stake_history: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction EmergencyUnstake
//* description: Creates the interface instruction `emergencyUnstake` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct EmergencyUnstake<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub validator_manager_authority: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_account: AccountInfo<'info>,
  pub stake_deposit_authority: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction PartialUnstake
//* description: Creates the interface instruction `partialUnstake` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct PartialUnstake<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub validator_manager_authority: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub stake_account: AccountInfo<'info>,
  pub stake_deposit_authority: AccountInfo<'info>,
  pub reserve_pda: AccountInfo<'info>,
  #[account(mut, signer)]
  pub split_stake_account: AccountInfo<'info>,
  #[account(mut, signer)]
  pub split_stake_rent_payer: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
  pub stake_history: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Instruction MergeStakes
//* description: Creates the interface instruction `mergeStakes` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct MergeStakes<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(mut)]
  pub stake_list: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
  #[account(mut)]
  pub destination_stake: AccountInfo<'info>,
  #[account(mut)]
  pub source_stake: AccountInfo<'info>,
  pub stake_deposit_authority: AccountInfo<'info>,
  pub stake_withdraw_authority: AccountInfo<'info>,
  #[account(mut)]
  pub operational_sol_account: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub stake_history: AccountInfo<'info>,
  pub stake_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: Marinade Finance's Account State
//* description: Generates the account `State` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[account]
pub struct State {
  pub msol_mint: Pubkey,
  pub admin_authority: Pubkey,
  pub operational_sol_account: Pubkey,
  pub treasury_msol_account: Pubkey,
  pub reserve_bump_seed: u8,
  pub msol_mint_authority_bump_seed: u8,
  pub rent_exempt_for_token_acc: u64,
  pub reward_fee: Fee,
  pub stake_system: StakeSystem,
  pub validator_system: ValidatorSystem,
  pub liq_pool: LiqPool,
  pub available_reserve_balance: u64,
  pub msol_supply: u64,
  pub msol_price: u64,
  pub circulating_ticket_count: u64,
  pub circulating_ticket_balance: u64,
  pub lent_from_reserve: u64,
  pub min_deposit: u64,
  pub min_withdraw: u64,
  pub staking_sol_cap: u64,
  pub emergency_cooling_down: u64,
}

/*/* content */*/

//* title: Marinade Finance's Account TicketAccountData
//* description: Generates the account `TicketAccountData` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[account]
pub struct TicketAccountData {
  pub state_address: Pubkey,
  pub beneficiary: Pubkey,
  pub lamports_amount: u64,
  pub created_epoch: u64,
}

/*/* content */*/

//* title: Marinade Finance's Type Fee
//* description: Generates the type `Fee` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Fee {
  pub basis_points: u32,
}

/*/* content */*/

//* title: Marinade Finance's Type InitializeData
//* description: Generates the type `InitializeData` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeData {
  pub admin_authority: Pubkey,
  pub validator_manager_authority: Pubkey,
  pub min_stake: u64,
  pub reward_fee: Fee,
  pub liq_pool: LiqPoolInitializeData,
  pub additional_stake_record_space: u32,
  pub additional_validator_record_space: u32,
  pub slots_for_stake_delta: u64,
}

/*/* content */*/

//* title: Marinade Finance's Type LiqPoolInitializeData
//* description: Generates the type `LiqPoolInitializeData` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct LiqPoolInitializeData {
  pub lp_liquidity_target: u64,
  pub lp_max_fee: Fee,
  pub lp_min_fee: Fee,
  pub lp_treasury_cut: Fee,
}

/*/* content */*/

//* title: Marinade Finance's Type ChangeAuthorityData
//* description: Generates the type `ChangeAuthorityData` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ChangeAuthorityData {
  pub admin: Option<Pubkey>,
  pub validator_manager: Option<Pubkey>,
  pub operational_sol_account: Option<Pubkey>,
  pub treasury_msol_account: Option<Pubkey>,
}

/*/* content */*/

//* title: Marinade Finance's Type ConfigLpParams
//* description: Generates the type `ConfigLpParams` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ConfigLpParams {
  pub min_fee: Option<Fee>,
  pub max_fee: Option<Fee>,
  pub liquidity_target: Option<u64>,
  pub treasury_cut: Option<Fee>,
}

/*/* content */*/

//* title: Marinade Finance's Type ConfigMarinadeParams
//* description: Generates the type `ConfigMarinadeParams` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ConfigMarinadeParams {
  pub rewards_fee: Option<Fee>,
  pub slots_for_stake_delta: Option<u64>,
  pub min_stake: Option<u64>,
  pub min_deposit: Option<u64>,
  pub min_withdraw: Option<u64>,
  pub staking_sol_cap: Option<u64>,
  pub liquidity_sol_cap: Option<u64>,
  pub auto_add_validator_enabled: Option<bool>,
}

/*/* content */*/

//* title: Marinade Finance's Type LiqPool
//* description: Generates the type `LiqPool` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct LiqPool {
  pub lp_mint: Pubkey,
  pub lp_mint_authority_bump_seed: u8,
  pub sol_leg_bump_seed: u8,
  pub msol_leg_authority_bump_seed: u8,
  pub msol_leg: Pubkey,
  pub lp_liquidity_target: u64,
  pub lp_max_fee: Fee,
  pub lp_min_fee: Fee,
  pub treasury_cut: Fee,
  pub lp_supply: u64,
  pub lent_from_sol_leg: u64,
  pub liquidity_sol_cap: u64,
}

/*/* content */*/

//* title: Marinade Finance's Type List
//* description: Generates the type `List` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct List {
  pub account: Pubkey,
  pub item_size: u32,
  pub count: u32,
  pub new_account: Pubkey,
  pub copied_count: u32,
}

/*/* content */*/

//* title: Marinade Finance's Type StakeRecord
//* description: Generates the type `StakeRecord` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct StakeRecord {
  pub stake_account: Pubkey,
  pub last_update_delegated_lamports: u64,
  pub last_update_epoch: u64,
  pub is_emergency_unstaking: u8,
}

/*/* content */*/

//* title: Marinade Finance's Type StakeSystem
//* description: Generates the type `StakeSystem` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct StakeSystem {
  pub stake_list: List,
  pub delayed_unstake_cooling_down: u64,
  pub stake_deposit_bump_seed: u8,
  pub stake_withdraw_bump_seed: u8,
  pub slots_for_stake_delta: u64,
  pub last_stake_delta_epoch: u64,
  pub min_stake: u64,
  pub extra_stake_delta_runs: u32,
}

/*/* content */*/

//* title: Marinade Finance's Type ValidatorRecord
//* description: Generates the type `ValidatorRecord` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ValidatorRecord {
  pub validator_account: Pubkey,
  pub active_balance: u64,
  pub score: u32,
  pub last_stake_delta_epoch: u64,
  pub duplication_flag_bump_seed: u8,
}

/*/* content */*/

//* title: Marinade Finance's Type ValidatorSystem
//* description: Generates the type `ValidatorSystem` of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ValidatorSystem {
  pub validator_list: List,
  pub manager_authority: Pubkey,
  pub total_validator_score: u32,
  pub total_active_balance: u64,
  pub auto_add_validator_enabled: u8,
}

/*/* content */*/

