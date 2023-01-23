// Based on https://github.dev/clockwork-xyz/examples/blob/31c97d14aae928f44b2f60d04abd5aa9edf4ce26/investments/programs/investments/

//* title: Openbook's Create Order
//* description: Create an Order PDA that can be executed in the Openbook protocol
//* platform: anchor
//* category: dex
//* display: vscode

use crate::instructions::dex::{order::*, openbook_dex::*};

/*/* content */*/
use {
	anchor_lang::{
			prelude::*,
			solana_program::{system_program, sysvar},
	},
	anchor_spl::token::{self, Mint, TokenAccount},
	std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct CreateOrder<'info> {
	#[account(address = anchor_spl::dex::ID)]
	pub dex_program: Program<'info, OpenBookDex>,
	#[account(
			init,
			seeds = [SEED_ORDER, payer.key().as_ref(), market.key().as_ref()],
			bump,
			payer = payer,
			space = 8 + size_of::<Order>(),
	)]
	pub order: Account<'info, Order>,
	/// CHECK: market
	pub market: AccountInfo<'info>,
	/// CHECK: mint_a
	pub mint_a: Account<'info, Mint>,
	#[account(mut)]
	pub payer: Signer<'info>,
	#[account(
			mut,
			associated_token::authority = payer,
			associated_token::mint = mint_a,
	)]
	pub payer_mint_a_token_account: Account<'info, TokenAccount>,
	#[account(address = sysvar::rent::ID)]
	pub rent: Sysvar<'info, Rent>,
	#[account(address = system_program::ID)]
	pub system_program: Program<'info, System>,
	#[account(address = anchor_spl::token::ID)]
	pub token_program: Program<'info, anchor_spl::token::Token>,
}

pub fn processor<'info>(
	ctx: Context<'_, '_, '_, 'info, CreateOrder<'info>>,
	amount: u64,
) -> Result<()> {
	let dex_program = &ctx.accounts.dex_program;
	let order = &mut ctx.accounts.order;
	let market = &ctx.accounts.market;
	let payer = &ctx.accounts.payer;
	let payer_mint_a_token_account = &mut ctx.accounts.payer_mint_a_token_account;
	let rent = &ctx.accounts.rent;
	let token_program = &ctx.accounts.token_program;
	let open_orders = ctx.remaining_accounts.get(0).unwrap();
	let bump = *ctx.bumps.get("order").unwrap();
	order.new(payer.key(), amount, market.key())?;

	token::approve(
			CpiContext::new(
					token_program.to_account_info(),
					token::Approve {
							to: payer_mint_a_token_account.to_account_info(),
							delegate: order.to_account_info(),
							authority: payer.to_account_info(),
					},
			),
			u64::MAX,
	)?;

	anchor_spl::dex::init_open_orders(CpiContext::new_with_signer(
			dex_program.to_account_info(),
			anchor_spl::dex::InitOpenOrders {
					authority: order.to_account_info(),
					market: market.to_account_info(),
					open_orders: open_orders.to_account_info(),
					rent: rent.to_account_info(),
			},
			&[&[
					SEED_ORDER,
					order.payer.as_ref(),
					order.market.as_ref(),
					&[bump],
			]],
	))?;

	Ok(())
}
/*/* content */*/