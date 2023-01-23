// Based on https://github.dev/clockwork-xyz/examples/blob/31c97d14aae928f44b2f60d04abd5aa9edf4ce26/investments/programs/investments/

//* title: Openbook's Account Order 
//* description: A order that can be executed on the Openbook protocol
//* platform: anchor
//* category: dex
//* display: vscode

/*/* content */*/
use {
	anchor_lang::{prelude::*, AnchorDeserialize},
	std::convert::TryFrom,
};

pub const SEED_ORDER: &[u8] = b"order";

#[account]
#[derive(Debug)]
pub struct Order {
	pub market: Pubkey,
	pub payer: Pubkey,
	pub amount: u64,
}

impl Order {
	pub fn pubkey(payer: Pubkey, market: Pubkey) -> Pubkey {
			Pubkey::find_program_address(
					&[SEED_ORDER, payer.as_ref(), market.as_ref()],
					&crate::ID,
			).0
	}
}

impl TryFrom<Vec<u8>> for Order {
	type Error = Error;
	fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
			Order::try_deserialize(&mut data.as_slice())
	}
}

pub trait OrderAccount {
	fn new(&mut self, payer: Pubkey, amount: u64, market: Pubkey) -> Result<()>;
}

impl OrderAccount for Account<'_, Order> {
	fn new(&mut self, payer: Pubkey, amount: u64, market: Pubkey) -> Result<()> {
			self.payer = payer;
			self.amount = amount;
			self.market = market;
			Ok(())
	}
}
/*/* content */*/