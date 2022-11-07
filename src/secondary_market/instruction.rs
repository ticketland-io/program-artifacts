use solana_sdk::{
  pubkey::Pubkey,
  slot_history::Slot,
};
use borsh::BorshSerialize;
use crate::ix::InstructionData;

/// This is the program instruction data i.e. fn params
#[derive(BorshSerialize)]
pub struct OperatorFillSellListingIx {
  pub event_id: [u8; 32],
  pub recipient: Pubkey,
}

impl InstructionData for OperatorFillSellListingIx {
  fn data(&self) -> Vec<u8> {
    let mut d = [235, 176, 160, 40, 55, 45, 31, 121].to_vec();
    d.append(&mut self.try_to_vec().expect("Should always serialize"));
    d
  }
}

#[derive(BorshSerialize)]
pub struct ReserveSellListingIx {
  pub sell_listing: Pubkey,
  pub duration: Slot,
  pub recipient: Pubkey,
}

impl InstructionData for ReserveSellListingIx {
  fn data(&self) -> Vec<u8> {
    let mut d = [151, 57, 123, 66, 106, 170, 93, 193].to_vec();
    d.append(&mut self.try_to_vec().expect("Should always serialize"));
    d
  }
}
