use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;

pub mod pda;
pub mod instruction;
pub mod account_data;

pub fn program_id() -> Pubkey {
  Pubkey::from_str("599YwRjALAKVj7z9bcBijrYHyNGLTJSjmJTzeyttnEFL").unwrap()
}
