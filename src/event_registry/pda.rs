use solana_sdk::{
  pubkey::Pubkey,
};
use solana_web3_rust::constants::METAPLEX_PROGRAM;

pub fn event(state: &Pubkey, event_id: &str) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"event", state.as_ref(), event_id.as_bytes()],
    &super::program_id(),
  )
}

pub fn event_nft(state: &Pubkey, event_id: &str) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"event_nft", state.as_ref(), event_id.as_bytes()],
    &super::program_id(),
  )
}

pub fn event_nft_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"event_nft_authority", state.as_ref()],
    &super::program_id(),
  )
}

// Metaplex PDA
pub fn event_nft_metadata(event_nft: &str) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"metadata", METAPLEX_PROGRAM.as_ref(), event_nft.as_bytes()],
    &METAPLEX_PROGRAM,
  )
}
