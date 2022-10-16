use borsh::{BorshDeserialize};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize)]
pub struct TicketMetadata {
  /// The actual token Mint account
  pub mint: Pubkey,

  /// The Event NFT that this NFT belongs to
  pub collection: Pubkey,

  /// The name of the asset
  pub name: String,

  /// The symbol for the asset
  pub symbol: String,
  
  /// URI pointing to JSON representing the asset
  pub uri: String,

  /// Indicated if the owner of this ticket attended the event
  pub attended: bool,
  
  /// The id of the event which this ticket belongs to
  pub event_id: [u8; 32],

  /// The seat index of this ticket
  pub seat_index: u32,

  /// The primary sale account containing information about how the sale of the ticket.
  /// This is useful as we can retrieve information such as ticket type etc.
  pub sale: Pubkey,

  /// The price the ticket is sold for. The reason we need to keep this explicitly is because
  /// the sale type can an auction and thus the Sale account can't tell us what price the ticket was
  /// sold for.
  pub price_sold: u64,

  /// The original owner of the Ticket NFT
  pub owner: Pubkey,
}
