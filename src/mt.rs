use solana_sdk::keccak::hashv;

/// Create the Leaf which is hashv(seat_index || "." || seat_name)
pub fn create_seat_leaf(seat_index: u32, seat_name: &String) -> [u8; 32] {
  hashv(&[
    seat_index.to_string().as_ref(),
    b".",
    seat_name.to_string().as_ref()
  ]).0
}
