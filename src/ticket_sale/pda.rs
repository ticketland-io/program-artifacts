use solana_sdk::{
  pubkey::Pubkey,
};

pub fn sale(
  state: &Pubkey,
  ticket_type_index: u8,
  event_id: [u8; 32],
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"sale",
      state.as_ref(),
      ticket_type_index.to_string().as_ref(),
      &event_id
    ],
    &super::program_id(),
  )
}

pub fn seat_verification(sale: &Pubkey, seat_index: u32, seat_name: &String) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"seat_verification",
      sale.as_ref(),
      seat_index.to_string().as_ref(),
      seat_name.as_ref(),  
    ],
    &super::program_id(),
  )
}

pub fn cpi_authority(state: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[b"ticket_sale:cpi_authority", state.as_ref()],
    &super::program_id(),
  )
}

pub fn seat_reservation(sale: &Pubkey, seat_index: u32, seat_name: &String) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"seat_reservation",
      sale.as_ref(),
      seat_index.to_string().as_ref(),
      seat_name.as_ref(),  
    ],
    &super::program_id(),
  )
}
