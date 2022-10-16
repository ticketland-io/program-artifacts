use solana_sdk::{
  pubkey::Pubkey,
};

pub fn ticket_nft(
  state: &Pubkey,
  seat_index: u32,
  event_id: [u8; 32],
  ticket_type_index: u8,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"ticket_nft", state.as_ref(), seat_index.to_string().as_ref(), &event_id, ticket_type_index.to_string().as_ref()],
    &super::program_id(),
  )
}
