use solana_sdk::{
  pubkey::Pubkey,
};
use borsh::BorshSerialize;

/// This is the program instruction data i.e. fn params
#[derive(BorshSerialize)]
pub struct UpdateEventNftUriIx {
  pub event_nft: Pubkey,
  pub new_uri: String,
}
