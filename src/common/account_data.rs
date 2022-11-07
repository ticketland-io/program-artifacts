use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::pubkey::Pubkey;

pub type Slot = u64;

#[derive(BorshSerialize, BorshDeserialize, Clone, Default)]
pub struct Reservation {
  pub valid_until: Slot,
  pub recipient: Pubkey,
}
