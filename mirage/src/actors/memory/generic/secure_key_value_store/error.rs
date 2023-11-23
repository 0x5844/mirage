use crate::actors::encryption::AesError;

#[derive(Debug)]
pub enum SecureStoreError {
    EncryptionError,
    DecryptionError,
}

impl From<SecureStoreError> for AesError {
    fn from(err: SecureStoreError) -> Self {
        AesError::SecureStoreError(err)
    }
}
