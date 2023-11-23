pub mod generic;
pub mod secure_memory_provider;

use crate::actors::encryption::AesError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Input {
    Buffer(Vec<u8>),
    Bit(u8),
}

impl Input {
    pub fn serialize(inputs: &[Input]) -> Result<Vec<u8>, AesError> {
        serde_json::to_vec(inputs).map_err(|e| AesError::SerializeError(e.to_string()))
    }

    pub fn deserialize(data: &[u8]) -> Result<Vec<Input>, AesError> {
        serde_json::from_slice(data).map_err(|e| AesError::DeserializeError(e.to_string()))
    }
}
