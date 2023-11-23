use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use crate::actors::encryption::decryptor::Decryptor;
use crate::actors::encryption::encryptor::Encryptor;

use self::error::SecureStoreError;

pub mod error;

pub struct SecureKeyValueStore {
    data: Vec<Option<(String, Vec<u8>)>>,
    encryptor: Arc<Encryptor>,
    decryptor: Arc<Decryptor>,
}

impl SecureKeyValueStore {
    pub fn new(encryptor: Arc<Encryptor>, decryptor: Arc<Decryptor>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(SecureKeyValueStore {
            data: Vec::new(),
            encryptor,
            decryptor,
        }))
    }

    pub fn set(&mut self, key: String, value: Vec<u8>) -> Result<(), SecureStoreError> {
        let encrypted_value = self
            .encryptor
            .encrypt(&value)
            .map_err(|_| SecureStoreError::EncryptionError)?;

        let index = self.calculate_unique_index(&key);
        self.ensure_capacity(index);
        self.data[index] = Some((key, encrypted_value));
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, SecureStoreError> {
        let index = self.calculate_unique_index(key);
        if let Some((_, encrypted_value)) = &self.data.get(index).unwrap_or(&None) {
            let decrypted_value = self
                .decryptor
                .decrypt(encrypted_value)
                .map_err(|_| SecureStoreError::DecryptionError)?;
            Ok(Some(decrypted_value))
        } else {
            Ok(None)
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn ensure_capacity(&mut self, index: usize) {
        if index >= self.data.len() {
            self.data.resize(index + 1, None);
        }
    }

    fn calculate_unique_index<Q: ?Sized>(&self, key: &Q) -> usize
    where
        Q: Hash,
    {
        let mut hasher = DefaultHasher::new(); // Replace with a custom hasher for better security
        key.hash(&mut hasher);
        hasher.finish() as usize % self.data.capacity()
    }
}

// Implement custom hasher as needed

// Usage example
// let secure_store = SecureKeyValueStore::new(Arc::new(Encryptor::new(...)), Arc::new(Decryptor::new(...)));
// let result = secure_store.lock().unwrap().set("key".to_string(), vec![1, 2, 3]);
