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

// We follow the same approach that Anchor does. The reason is that we need to provide the instuction code as part of the tx 
// data so when it hits Anchor program it know which ix processor to call.
// If we run anchor expand we can see similar code to this
impl InstructionData for SetTicketAttendedIx {
  fn data(&self) -> Vec<u8> {
    // Check how this is calculate here https://github.com/coral-xyz/anchor/blob/698426033052781988cd7980249726501ae08bdc/lang/syn/src/codegen/program/instruction.rs#L137
    // Instead we just copied the value from the code Anchor has generated after running Anchor expand.
    let mut d = [167, 44, 23, 56, 62, 140, 237, 135].to_vec();
    d.append(&mut self.try_to_vec().expect("Should always serialize"));
    d
}
}
