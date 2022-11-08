use solana_sdk::{
  pubkey::Pubkey,
};
use borsh::BorshSerialize;
use crate::ix::InstructionData;

/// This is the program instruction data i.e. fn params
#[derive(BorshSerialize)]
pub struct SetTicketAttendedIx {
  pub ticket_metadata: Pubkey,
}

impl InstructionData for SetTicketAttendedIx {
  fn data(&self) -> Vec<u8> {
    let mut d = [167, 44, 23, 56, 62, 140, 237, 135].to_vec();
    d.append(&mut self.try_to_vec().expect("Should always serialize"));
    d
}
}
