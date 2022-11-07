use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::{
  pubkey::Pubkey,
  slot_history::Slot,
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Default)]
pub struct Reservation {
  pub valid_until: Slot,
  pub recipient: Pubkey,
}
