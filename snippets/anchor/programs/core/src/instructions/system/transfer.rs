//* title: Openbook's Update Order
//* description: Updates an Order PDA that can be executed in the Openbook protocol
//* platform: anchor
//* category: dex
//* display: vscode

use crate::*;

/*/* content */*/
use anchor_lang::{prelude::*, solana_program::system_program};

#[derive(Accounts)]
#[instruction(swap_amount: u64)]
pub struct UpdateOrder<'info> {
    #[account(
			mut,
			seeds = [SEED_ORDER, payer.key().as_ref(), market.key().as_ref()],
			bump,
			has_one = market
	)]
    pub order: Account<'info, Order>,
    /// CHECK: market
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn processor<'info>(ctx: Context<UpdateOrder<'info>>, amount: u64) -> Result<()> {
    let order = &mut ctx.accounts.order;

    order.amount = amount;

    Ok(())
}
/*/* content */*/