use solana_sdk::{
  pubkey::Pubkey,
};

pub fn market(state: &Pubkey, event_id: &str) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"market", state.as_ref(), event_id.as_bytes()],
    &super::program_id(),
  )
}

pub fn cpi_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"market:cpi_authority", state.as_ref()],
    &super::program_id(),
  )
}

pub fn sell_listing(
  state: &Pubkey,
  event_id: &str,
  ticket_metadata: &Pubkey,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"sell_listing", state.as_ref(), event_id.as_bytes(), ticket_metadata.as_ref()],
    &super::program_id(),
  )
}
