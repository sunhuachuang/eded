use crate::Result;

#[cfg(feature = "memory-db")]
pub mod memory;

#[cfg(feature = "rocks-db")]
pub mod rocks;

pub trait Storage {
    fn get(&self, key: &[u8]) -> Result<Vec<u8>>;

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;

    fn del(&mut self, key: &[u8]) -> Result<Vec<u8>>;
}
