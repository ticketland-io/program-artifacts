use solana_sdk::{
  pubkey::Pubkey,
};
use borsh::BorshSerialize;

/// This is the program instruction data i.e. fn params
#[derive(BorshSerialize)]
pub struct UpdateEventNftUriIx {
  event_nft: Pubkey,
  new_uri: String,
}
