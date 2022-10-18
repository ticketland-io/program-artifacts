use std::ops::Deref;

pub struct EventId(String);

impl Deref for EventId {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
