use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnP");

//* title: InitializeEscrow Escrow Interface
//* description: Creates the interface of the instruction initializeEscrow of the escrow program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/
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

/*/* content */*/

//* title: CancelEscrow Escrow Interface
//* description: Creates the interface of the instruction cancelEscrow of the escrow program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/
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

/*/* content */*/

//* title: Exchange Escrow Interface
//* description: Creates the interface of the instruction exchange of the escrow program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/
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

/*/* content */*/

//* title: EscrowAccount Escrow Interface
//* description: Creates the interface of the instruction EscrowAccount of the escrow program
//* platform: anchor
//* category: interfaces
//* prefix: program
//* display: vscode
		
/*/* content */*/
#[account]
pub struct EscrowAccount {
  pub initializer_key: Pubkey,
  pub initializer_deposit_token_account: Pubkey,
  pub initializer_receive_token_account: Pubkey,
  pub initializer_amount: u64,
  pub taker_amount: u64,
}

/*/* content */*/

