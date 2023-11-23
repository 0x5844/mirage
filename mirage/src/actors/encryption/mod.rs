use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io;

use aes::cipher::InvalidLength;
use aes_gcm::Error as AesGcmError;

use crate::actors::memory::secure_key_value_store::SecureStoreError;
use crate::tools::config::EncryptionLevel; // Update path as necessary

pub mod decryptor;
pub mod encryptor;

pub const KEY_LENGTH: usize = 12;

#[derive(Debug)]
pub enum AesError {
    AesGcmError(AesGcmError),
    SecureStoreError(SecureStoreError), // New variant for SecureStoreError
    InvalidKeyLength,
    IoError(io::Error),
    SerializeError(String),
    DeserializeError(String),
}

impl Error for AesError {}

impl fmt::Display for AesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AesError::AesGcmError(err) => write!(f, "AES GCM encryption/decryption error: {}", err),
            AesError::SecureStoreError(err) => write!(f, "Secure store error: {:?}", err), // Display for SecureStoreError
            AesError::InvalidKeyLength => {
                write!(f, "Invalid key length, must be {} bytes", KEY_LENGTH)
            }
            AesError::IoError(err) => write!(f, "IO error: {}", err),
            AesError::SerializeError(err) => write!(f, "Serialization error: {}", err),
            AesError::DeserializeError(err) => write!(f, "Deserialization error: {}", err),
        }
    }
}

impl From<AesGcmError> for AesError {
    fn from(err: AesGcmError) -> Self {
        AesError::AesGcmError(err)
    }
}

impl From<InvalidLength> for AesError {
    fn from(_err: InvalidLength) -> Self {
        AesError::InvalidKeyLength
    }
}

impl From<io::Error> for AesError {
    fn from(err: io::Error) -> Self {
        AesError::IoError(err)
    }
}

pub enum Input {
    File(PathBuf),
    Buffer(Vec<u8>),
    Bit(u8),
}

#[derive(Clone)]
pub struct CryptoBase {
    is_alive: bool,
    level: EncryptionLevel,
    key: Arc<Vec<u8>>,
    nonce: Arc<Vec<u8>>,
    has_parallel_processing: bool,
    runtime_handle: Arc<tokio::runtime::Handle>,
}
