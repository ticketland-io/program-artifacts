use solana_sdk::{
  pubkey::Pubkey,
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
