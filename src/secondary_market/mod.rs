use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;

pub mod instruction;
pub mod pda;

pub fn program_id() -> Pubkey {
  Pubkey::from_str("ECRCx1XuhFC1DatvsvMu6nwrHQzMo3h41X2vKdvD7S5f").unwrap()
}
