use solana_sdk::{
  pubkey::Pubkey,
};

pub fn event_nft_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"event_nft_authority", state.as_ref()],
    &super::program_id(),
  )
}
