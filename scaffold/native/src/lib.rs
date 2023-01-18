pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]

solana_program::declare_id!("6Tsgz5SziqCqmeAJATShqiU7GqBsQp9JyPXUi59zvka5");
