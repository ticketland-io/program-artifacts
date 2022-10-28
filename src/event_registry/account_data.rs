use eyre::{Result, Report};

pub struct EventId(pub String);

impl EventId {
  pub fn db_val(&self) -> String {
    self.0.clone()
  }

  pub fn val(&self) -> String {
    self.0.replace("-", "").clone()
  }

  pub fn bytes(&self) -> Result<[u8; 32]> {
    let val: [u8; 32] = self.val().into_bytes()
    .try_into()
    .map_err(|error| Report::msg(format!("{:?}", error)))?;
    
    Ok(val)
  }
}
