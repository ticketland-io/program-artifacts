use solana_sdk::keccak::hashv;
use solana_web3_rust::crypto::mt::MerkleTree;

/// Create the Leaf which is hashv(seat_index || "." || seat_name)
pub fn create_seat_leaf(seat_index: u32, seat_name: &String) -> [u8; 32] {
  hashv(&[
    seat_index.to_string().as_ref(),
    b".",
    seat_name.to_string().as_ref()
  ]).0
}

pub fn create_ticket_type_mt(seat_indexes: Vec<(u32, u32)>, n_tickets: u32) -> MerkleTree {
  let null_leaf = MerkleTree::get_null_leaf();
  let mut seats = vec![null_leaf; n_tickets as usize];

  for seat_range in seat_indexes {
    for i in seat_range.0..seat_range.1 {
      // This handles the merkle tree of a simple venue i.e. seat index and seat name are the same
      seats[i as usize] = create_seat_leaf(i, &i.to_string());
    }
  }
  
  MerkleTree::new(seats.to_vec())
}
