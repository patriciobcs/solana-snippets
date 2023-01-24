use anchor_lang::prelude::*;

//* title: Get Account Info
//* description: Get an account from an instruction context
//* platform: anchor
//* category: system
//* prefix: gbump

#[account]
pub struct PDA {
  pub bump: u8,
}

#[derive(Accounts)]
pub struct GetAccountInfo<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
      init,
      payer = user,
      space = 8 + 1, 
      seeds = [b"__pda__"], 
      bump
  )]
  pub __pda__: Account<'info, PDA>,
  pub system_program: Program<'info, System>,
}

pub fn processor(ctx: Context<GetAccountInfo>) -> Result<()> {
    /*/* content */*/
    let __pda__ = &mut ctx.accounts.__pda__;
    /*/* content */*/
    Ok(())
}