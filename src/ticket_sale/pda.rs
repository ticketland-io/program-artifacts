use solana_sdk::{
  pubkey::Pubkey,
};

pub fn seat_verification(sale: &str, seat_index: u32, seat_name: &String) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      b"seat_verification",
      sale.as_bytes(),
      seat_index.to_string().as_ref(),
      seat_name.as_ref(),  
    ],
    &super::program_id(),
  )
}
