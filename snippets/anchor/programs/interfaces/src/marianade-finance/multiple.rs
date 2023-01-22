use anchor_lang::prelude::*;

declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

//* title: Initialize Marinade_finance Interface
//* description: Creates the interface of the instruction initialize of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct LiqPool<'info> {
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
  pub liq_pool: LiqPool<'info>,
  pub treasury_msol_account: AccountInfo<'info>,
  pub clock: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

/*/* content */*/

//* title: ChangeAuthority Marinade_finance Interface
//* description: Creates the interface of the instruction changeAuthority of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: AddValidator Marinade_finance Interface
//* description: Creates the interface of the instruction addValidator of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: RemoveValidator Marinade_finance Interface
//* description: Creates the interface of the instruction removeValidator of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: SetValidatorScore Marinade_finance Interface
//* description: Creates the interface of the instruction setValidatorScore of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: ConfigValidatorSystem Marinade_finance Interface
//* description: Creates the interface of the instruction configValidatorSystem of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: Deposit Marinade_finance Interface
//* description: Creates the interface of the instruction deposit of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: DepositStakeAccount Marinade_finance Interface
//* description: Creates the interface of the instruction depositStakeAccount of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: LiquidUnstake Marinade_finance Interface
//* description: Creates the interface of the instruction liquidUnstake of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: AddLiquidity Marinade_finance Interface
//* description: Creates the interface of the instruction addLiquidity of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: RemoveLiquidity Marinade_finance Interface
//* description: Creates the interface of the instruction removeLiquidity of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: ConfigLp Marinade_finance Interface
//* description: Creates the interface of the instruction configLp of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: ConfigMarinade Marinade_finance Interface
//* description: Creates the interface of the instruction configMarinade of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: OrderUnstake Marinade_finance Interface
//* description: Creates the interface of the instruction orderUnstake of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: Claim Marinade_finance Interface
//* description: Creates the interface of the instruction claim of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: StakeReserve Marinade_finance Interface
//* description: Creates the interface of the instruction stakeReserve of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: UpdateActive Marinade_finance Interface
//* description: Creates the interface of the instruction updateActive of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct Common<'info> {
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
  pub common: Common<'info>,
  #[account(mut)]
  pub validator_list: AccountInfo<'info>,
}

/*/* content */*/

//* title: UpdateDeactivated Marinade_finance Interface
//* description: Creates the interface of the instruction updateDeactivated of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/
#[derive(Accounts)]
pub struct Common<'info> {
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
  pub common: Common<'info>,
  #[account(mut)]
  pub operational_sol_account: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
}

/*/* content */*/

//* title: DeactivateStake Marinade_finance Interface
//* description: Creates the interface of the instruction deactivateStake of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: EmergencyUnstake Marinade_finance Interface
//* description: Creates the interface of the instruction emergencyUnstake of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: PartialUnstake Marinade_finance Interface
//* description: Creates the interface of the instruction partialUnstake of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: MergeStakes Marinade_finance Interface
//* description: Creates the interface of the instruction mergeStakes of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: State Marinade_finance Interface
//* description: Creates the interface of the instruction State of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

//* title: TicketAccountData Marinade_finance Interface
//* description: Creates the interface of the instruction TicketAccountData of the marinade_finance program
//* platform: anchor
//* category: interfaces
//* prefix: program
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

