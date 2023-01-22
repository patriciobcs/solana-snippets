use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
  #[account(mut, signer)]
  pub initializer: AccountInfo<'info>,
  #[account(mut)]
  pub initializer_deposit_token_account: AccountInfo<'info>,
  pub initializer_receive_token_account: AccountInfo<'info>,
  #[account(mut, signer)]
  pub escrow_account: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
  pub initializer: AccountInfo<'info>,
  #[account(mut)]
  pub pda_deposit_token_account: AccountInfo<'info>,
  pub pda_account: AccountInfo<'info>,
  #[account(mut)]
  pub escrow_account: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
  #[account(signer)]
  pub taker: AccountInfo<'info>,
  #[account(mut)]
  pub taker_deposit_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub taker_receive_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub pda_deposit_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub initializer_receive_token_account: AccountInfo<'info>,
  #[account(mut)]
  pub initializer_main_account: AccountInfo<'info>,
  #[account(mut)]
  pub escrow_account: AccountInfo<'info>,
  pub pda_account: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
}

#[account]
pub struct EscrowAccount {
  pub initializer_key: Pubkey,
  pub initializer_deposit_token_account: Pubkey,
  pub initializer_receive_token_account: Pubkey,
  pub initializer_amount: u64,
  pub taker_amount: u64,
}

