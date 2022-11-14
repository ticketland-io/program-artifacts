use std::ops::{Deref,DerefMut};
use borsh::{BorshSerialize, BorshDeserialize};
use crate::common::account_data::Reservation;

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
  pub name: String,
  pub n_tickets: u32,
  pub sale_type: SaleType,
  pub sale_start_time: i64,
  pub sale_end_time: i64,
  pub merkle_root: [u8; 32],
  pub seat_range: SeatRange,
}

#[derive(BorshDeserialize)]
pub struct SeatVerification {
  pub bump: u8,
  pub verified: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SeatReservation(pub Reservation);

impl Deref for SeatReservation {
  type Target = Reservation;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for SeatReservation {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EventCapacity {
  pub event_id: [u8; 32],
  pub is_initialized: bool,
  pub available_tickets: u32,

  // A bitmap which has n_tickets bits that represent each seat
  // By default all bits are 0. When a ticket at ticket index N (Nth bit) is purchased
  // then the bit is flipped to 1 indicating that the seat is not available
  // Bitmap allows us to store compact data e.g With 12500 bytes we can represent up to 12500 * 8 = 100_000 seats
  pub seats: Vec<u8>,
}
