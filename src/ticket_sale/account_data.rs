use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Sale {
  pub bump: u8,

  /// The unique id of the event which this sale is part of
  pub event_id: [u8; 32],

  /// A unique index that will differentiate multiple sales of one single event
  pub ticket_type_index: u8,

  /// The ticket type that decides the sale mechanism
  pub ticket_type: TicketType,
}


#[derive(BorshSerialize, BorshDeserialize)]
pub enum SaleType {
  Free,
  FixedPrice {amount: u64},
  Refundable {amount: u64},
  DutchAuction {
    start_price: u64,
    end_price: u64,
    curve_length: u16,
    drop_interval: u16,
  }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SeatRange {
  pub l: u32,
  pub r: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TicketType {
  pub n_tickets: u32,
  pub sale_type: SaleType,
  pub sale_start_time: i64,
  pub sale_end_time: i64,
  pub merkle_root: [u8; 32],
  pub seat_range: SeatRange,
}
