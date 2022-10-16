use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;

pub mod account_data;

pub fn program_id() -> Pubkey {
  Pubkey::from_str("6banhWF9WKQk26NtgX6TqHNmAKgvX9aJmgaBvPmYCCK3").unwrap()
}
