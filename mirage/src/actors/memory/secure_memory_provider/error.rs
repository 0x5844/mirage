use crate::actors::memory::generic::secure_key_value_store::error::SecureStoreError;

#[derive(Debug)]
pub enum SecureMemoryProviderError {
    StoreError(SecureStoreError),
}

impl From<SecureStoreError> for SecureMemoryProviderError {
    fn from(err: SecureStoreError) -> Self {
        SecureMemoryProviderError::StoreError(err)
    }
}
