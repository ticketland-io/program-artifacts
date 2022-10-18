use borsh::BorshSerialize;
pub trait InstructionData: BorshSerialize {
  fn data(&self) -> Vec<u8>;
}
