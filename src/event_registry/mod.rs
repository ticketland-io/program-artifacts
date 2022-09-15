use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;

pub mod instruction;
pub mod pda;

pub fn program_id() -> Pubkey {
  Pubkey::from_str("TGfdMZj2HoSwdFR5zUAKr8H72XYJ85GQ7my5yZTHGKE").unwrap()
}
