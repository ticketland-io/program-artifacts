use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Default)]
pub struct Reservation {
  pub valid_until: Slot,
  pub recipient: Pubkey,
}
