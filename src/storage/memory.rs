use std::collections::HashMap;

use crate::{Error, Result};

use super::Storage;

pub struct MemoryDB {
    inner: HashMap<Vec<u8>, Vec<u8>>,
}

impl Storage for MemoryDB {
    fn get(&self, key: &[u8]) -> Result<Vec<u8>> {
        self.inner
            .get(key)
            .map(|v| v.to_vec())
            .ok_or(Error::StorageMissing)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.inner.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<Vec<u8>> {
        self.inner.remove(key).ok_or(Error::StorageMissing)
    }
}
