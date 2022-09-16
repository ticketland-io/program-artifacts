use solana_sdk::{
  pubkey::Pubkey,
};
use borsh::BorshSerialize;
use crate::ix::InstructionData;

/// This is the program instruction data i.e. fn params
#[derive(BorshSerialize)]
pub struct UpdateEventNftUriIx {
  pub event_nft: Pubkey,
  pub new_uri: String,
}

// We follow the same approach that Anchor does. The reason is that we need to provide the instuction code as part of the tx 
// data so when it hits Anchor program it know which ix processor to call.
// If we run anchor expand we can see similar code to this
impl InstructionData for UpdateEventNftUriIx {
  fn data(&self) -> Vec<u8> {
    // Check how this is calculate here https://github.com/coral-xyz/anchor/blob/698426033052781988cd7980249726501ae08bdc/lang/syn/src/codegen/program/instruction.rs#L137
    // Instead we just copied the value from the code Anchor has generated after running Anchor expand.
    let mut d = [246, 207, 102, 214, 220, 81, 202, 182].to_vec();
    d.append(&mut self.try_to_vec().expect("Should always serialize"));
    d
}
}
