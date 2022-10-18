pub struct EventId(pub String);

impl EventId{
  pub fn db_val(&self) -> String {
    self.0.clone()
  }

  pub fn val(&self) -> String {
    self.0.replace("-", "").clone()
  }
}
