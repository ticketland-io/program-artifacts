use std::ops::{Deref,DerefMut};
use borsh::{BorshSerialize, BorshDeserialize};
use crate::common::account_data::Reservation;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SellListingReservation(pub Reservation);

impl Deref for SellListingReservation {
  type Target = Reservation;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for SellListingReservation {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
