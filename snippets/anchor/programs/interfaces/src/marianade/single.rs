//* title: All Marinade Finance Interfaces
//* description: Creates the interface of the `Marinade Finance` program
//* platform: anchor
//* category: interfaces
//* display: vscode
		
/*/* content */*/
use anchor_lang::prelude::*;

declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

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

#[derive(Accounts)]
pub struct ChangeAuthority<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub admin_authority: AccountInfo<'info>,
}

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

#[derive(Accounts)]
pub struct SetValidatorScore<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub manager_authority: AccountInfo<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConfigValidatorSystem<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub manager_authority: AccountInfo<'info>,
}

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

#[derive(Accounts)]
pub struct ConfigLp<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub admin_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConfigMarinade<'info> {
  #[account(mut)]
  pub state: AccountInfo<'info>,
  #[account(signer)]
  pub admin_authority: AccountInfo<'info>,
}

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

#[account]
pub struct TicketAccountData {
  pub state_address: Pubkey,
  pub beneficiary: Pubkey,
  pub lamports_amount: u64,
  pub created_epoch: u64,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Fee {
  pub basis_points: u32,
}

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

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct LiqPoolInitializeData {
  pub lp_liquidity_target: u64,
  pub lp_max_fee: Fee,
  pub lp_min_fee: Fee,
  pub lp_treasury_cut: Fee,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ChangeAuthorityData {
  pub admin: Option<Pubkey>,
  pub validator_manager: Option<Pubkey>,
  pub operational_sol_account: Option<Pubkey>,
  pub treasury_msol_account: Option<Pubkey>,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ConfigLpParams {
  pub min_fee: Option<Fee>,
  pub max_fee: Option<Fee>,
  pub liquidity_target: Option<u64>,
  pub treasury_cut: Option<Fee>,
}

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

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct List {
  pub account: Pubkey,
  pub item_size: u32,
  pub count: u32,
  pub new_account: Pubkey,
  pub copied_count: u32,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct StakeRecord {
  pub stake_account: Pubkey,
  pub last_update_delegated_lamports: u64,
  pub last_update_epoch: u64,
  pub is_emergency_unstaking: u8,
}

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

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ValidatorRecord {
  pub validator_account: Pubkey,
  pub active_balance: u64,
  pub score: u32,
  pub last_stake_delta_epoch: u64,
  pub duplication_flag_bump_seed: u8,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct ValidatorSystem {
  pub validator_list: List,
  pub manager_authority: Pubkey,
  pub total_validator_score: u32,
  pub total_active_balance: u64,
  pub auto_add_validator_enabled: u8,
}

/*/* content */*/

