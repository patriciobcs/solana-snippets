// Based on https://github.dev/clockwork-xyz/examples/blob/31c97d14aae928f44b2f60d04abd5aa9edf4ce26/investments/programs/investments/

//* title: Openbook's Execute Order
//* description: Execute an Order PDA that can be executed in the Openbook protocol
//* platform: anchor
//* category: dex
//* display: vscode

use super::*;

/*/* content */*/
use {
    anchor_lang::{prelude::*, __private::bytemuck::Contiguous, solana_program::{system_program, sysvar}},
    anchor_spl::{ 
        dex::{serum_dex::{instruction::SelfTradeBehavior, matching::{OrderType, Side}}, NewOrderV3},
        token::{Token, TokenAccount},
    },
    std::num::NonZeroU64,
};

#[derive(Accounts)]
pub struct ExecuteOrder<'info> {
    #[account(address = anchor_spl::dex::ID)]
    pub dex_program: Program<'info, OpenBookDex>,
    #[account(
        seeds = [
            SEED_ORDER, 
            order.payer.key().as_ref(), 
            order.market.key().as_ref(), 
        ], 
        bump,
    )]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        associated_token::authority = payer,
        associated_token::mint = order,
    )]
    pub payer_mint_a_token_account: Account<'info, TokenAccount>,
    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

pub fn processor<'info>(ctx: Context<'_, '_, '_, 'info, ExecuteOrder<'info>>) -> Result<()> {
    let dex_program = &ctx.accounts.dex_program;
    let order = &ctx.accounts.order;
    let payer_mint_a_token_account= &mut ctx.accounts.payer_mint_a_token_account;
    let rent = &ctx.accounts.rent;
    let token_program = &ctx.accounts.token_program;
    let market = ctx.remaining_accounts.get(0).unwrap();
    let mint_a_vault = ctx.remaining_accounts.get(1).unwrap();
    let mint_b_vault = ctx.remaining_accounts.get(2).unwrap();
    let request_queue = ctx.remaining_accounts.get(3).unwrap();
    let event_queue = ctx.remaining_accounts.get(4).unwrap();
    let market_bids = ctx.remaining_accounts.get(5).unwrap();
    let market_asks = ctx.remaining_accounts.get(6).unwrap();
    let open_orders = ctx.remaining_accounts.get(7).unwrap();
    let bump = *ctx.bumps.get("order").unwrap();

    anchor_spl::dex::new_order_v3(
        CpiContext::new_with_signer(
            dex_program.to_account_info(),
            NewOrderV3 {
                market: market.to_account_info(),
                coin_vault: mint_b_vault.to_account_info(),
                pc_vault: mint_a_vault.to_account_info(),
                request_queue: request_queue.to_account_info(),
                event_queue: event_queue.to_account_info(),
                market_bids: market_bids.to_account_info(),
                market_asks: market_asks.to_account_info(),
                open_orders: open_orders.to_account_info(),
                order_payer_token_account: payer_mint_a_token_account.to_account_info(),
                open_orders_authority: order.to_account_info(),
                token_program: token_program.to_account_info(),
                rent: rent.to_account_info(),
            },
            &[&[
                SEED_ORDER,
                order.payer.as_ref(),
                order.market.as_ref(),
                &[bump],
            ]],
        ),
        Side::Bid,
        NonZeroU64::new(NonZeroU64::MAX_VALUE).unwrap(),
        NonZeroU64::new(NonZeroU64::MAX_VALUE).unwrap(),
        NonZeroU64::new(order.amount).unwrap(),
        SelfTradeBehavior::DecrementTake,
        OrderType::Limit,
        019269,
        std::u16::MAX,
    )?;

    Ok(())
}
/*/* content */*/