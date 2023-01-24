// Based on https://github.dev/clockwork-xyz/examples/blob/31c97d14aae928f44b2f60d04abd5aa9edf4ce26/investments/programs/investments/

//* title: Openbook's Swap
//* description: Swap two tokens on the Openbook protocol
//* platform: anchor
//* category: dex
//* display: vscode

use super::*;

/*/* content */*/
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::dex::serum_dex::matching::Side;

// The only constraint imposed on these accounts is that the market's base
// currency mint is not equal to the quote currency's. All other checks are
// done by the DEX on CPI.
#[derive(Accounts)]
pub struct Swap<'info> {
    pub market: MarketAccounts<'info>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub pc_wallet: AccountInfo<'info>,
    // Programs.
    pub dex_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    // Sysvars.
    pub rent: AccountInfo<'info>,
}

impl<'info> From<&Swap<'info>> for OrderbookClient<'info> {
    fn from(accounts: &Swap<'info>) -> OrderbookClient<'info> {
        OrderbookClient {
            market: accounts.market.clone(),
            authority: accounts.authority.clone(),
            pc_wallet: accounts.pc_wallet.clone(),
            dex_program: accounts.dex_program.clone(),
            token_program: accounts.token_program.clone(),
            rent: accounts.rent.clone(),
        }
    }
}

pub fn processor<'info>(
    ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
    side: Side,
    amount: u64,
    min_expected_swap_amount: u64,
) -> Result<()> {
    // Optional referral account (earns a referral fee).
    let referral = ctx.remaining_accounts.iter().next().map(Clone::clone);

    // Side determines swap direction.
    let (from_token, to_token) = match side {
        Side::Bid => (&ctx.accounts.pc_wallet, &ctx.accounts.market.coin_wallet),
        Side::Ask => (&ctx.accounts.market.coin_wallet, &ctx.accounts.pc_wallet),
    };

    // Token balances before the trade.
    let from_amount_before = token::accessor::amount(from_token)?;
    let to_amount_before = token::accessor::amount(to_token)?;

    // Execute trade.
    let orderbook: OrderbookClient<'info> = (&*ctx.accounts).into();
    match side {
        Side::Bid => orderbook.buy(amount, referral.clone())?,
        Side::Ask => orderbook.sell(amount, referral.clone())?,
    };
    orderbook.settle(referral)?;

    // Token balances after the trade.
    let from_amount_after = token::accessor::amount(from_token)?;
    let to_amount_after = token::accessor::amount(to_token)?;

    //  Calculate the delta, i.e. the amount swapped.
    let from_amount = from_amount_before.checked_sub(from_amount_after).unwrap();
    let to_amount = to_amount_after.checked_sub(to_amount_before).unwrap();

    if to_amount < min_expected_swap_amount {
        return Err(ProgramError::InvalidAccountData.into());
    }

    emit!(DidSwap {
        authority: *ctx.accounts.authority.key,
        given_amount: amount,
        min_expected_swap_amount,
        from_amount,
        to_amount,
        spill_amount: 0,
        from_mint: token::accessor::mint(from_token)?,
        to_mint: token::accessor::mint(to_token)?,
        quote_mint: match side {
            Side::Bid => token::accessor::mint(from_token)?,
            Side::Ask => token::accessor::mint(to_token)?,
        },
    });

    Ok(())
}

// Event emitted when a swap occurs for two base currencies on two different
// markets (quoted in the same token).
#[event]
pub struct DidSwap {
    // User given (max) amount to swap.
    pub given_amount: u64,
    // The minimum amount of the *to* token expected to be received from
    // executing the swap.
    pub min_expected_swap_amount: u64,
    // Amount of the `from` token sold.
    pub from_amount: u64,
    // Amount of the `to` token purchased.
    pub to_amount: u64,
    // Amount of the quote currency accumulated from the swap.
    pub spill_amount: u64,
    // Mint sold.
    pub from_mint: Pubkey,
    // Mint purchased.
    pub to_mint: Pubkey,
    // Mint of the token used as the quote currency in the two markets used
    // for swapping.
    pub quote_mint: Pubkey,
    // User that signed the transaction.
    pub authority: Pubkey,
}
/*/* content */*/