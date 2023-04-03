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
    let mut d = [159, 194, 146, 186, 145, 129, 56, 4].to_vec();
    d.append(&mut self.try_to_vec().expect("Should always serialize"));
    d
}
}
