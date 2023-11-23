use super::generic::secure_key_value_store::error::SecureStoreError;
use super::generic::secure_key_value_store::SecureKeyValueStore;
use super::Input;
use crate::actors::encryption::decryptor::Decryptor;
use crate::actors::encryption::encryptor::Encryptor;
use crate::actors::encryption::AesError;
use crate::actors::Actor;
use crate::utils::key_generator::{generate_aes_key, generate_nonce};
use std::sync::{Arc, Mutex};

pub mod error;

pub struct SecureMemoryProvider {
    fragments: Arc<Mutex<SecureKeyValueStore>>,
    encryptor: Arc<Encryptor>,
    decryptor: Arc<Decryptor>,
}

trait Encryption {
    fn encrypt_fragment(&self, id: String) -> Result<(), AesError>;
    fn decrypt_fragment(&self, id: String) -> Result<Option<Vec<u8>>, AesError>;
}

trait Mitigation {
    fn scramble_fragment(&mut self);
    fn tamper_adjacent_blocks(&mut self);
    fn shuffle_fragments(&mut self);
    fn move_fragment(&mut self);
}

impl Encryption for SecureMemoryProvider {
    fn encrypt_fragment(&self, id: String) -> Result<(), AesError> {
        let mut fragments = self.fragments.lock().unwrap();
        if let Some(data) = fragments.get(&id).map_err(AesError::from)? {
            let encrypted_data = self.encryptor.encrypt(&data).map_err(AesError::from)?;
            fragments.set(id, encrypted_data).map_err(AesError::from)?;
        }
        Ok(())
    }

    fn decrypt_fragment(&self, id: String) -> Result<Option<Vec<u8>>, AesError> {
        let fragments = self.fragments.lock().unwrap();
        if let Some(encrypted_data) = fragments.get(&id).map_err(AesError::from)? {
            let decrypted_data = self
                .decryptor
                .decrypt(&encrypted_data)
                .map_err(AesError::from)?;
            Ok(Some(decrypted_data))
        } else {
            Ok(None)
        }
    }
}

impl Mitigation for SecureMemoryProvider {
    fn scramble_fragment(&mut self) {
        todo!()
    }

    fn tamper_adjacent_blocks(&mut self) {
        todo!()
    }

    fn shuffle_fragments(&mut self) {
        todo!()
    }

    fn move_fragment(&mut self) {
        todo!()
    }
}

impl Actor<Input> for SecureMemoryProvider {
    fn is_alive(&self) -> bool {
        true
    }

    fn run(&self, input: Option<Input>) {
        // Implementation goes here...
    }
}

impl SecureMemoryProvider {
    pub fn new() -> Self {
        let key = generate_aes_key();
        let nonce = generate_nonce();
        let runtime_handle = Arc::new(tokio::runtime::Handle::try_current().unwrap());

        let encryptor = Arc::new(Encryptor::new(
            None,
            key.clone(),
            nonce.clone(),
            runtime_handle.clone(),
        ));
        let decryptor = Arc::new(Decryptor::new(
            None,
            key.clone(),
            nonce.clone(),
            runtime_handle.clone(),
        ));

        SecureMemoryProvider {
            fragments: SecureKeyValueStore::new(encryptor.clone(), decryptor.clone()),
            encryptor,
            decryptor,
        }
    }

    pub fn get(&self, id: &str) -> Result<Option<Vec<Input>>, AesError> {
        let fragments = self.fragments.lock().unwrap();
        if let Some(encrypted_data) = fragments.get(id)? {
            let data: Vec<Input> = Input::deserialize(&encrypted_data)?; // Ensure correct type
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    pub fn set(&mut self, id: String, data: Vec<Input>) -> Result<(), SecureStoreError> {
        let serialized_data: Vec<u8> = Input::serialize(&data)?; // Ensure correct type
        let mut fragments = self.fragments.lock().unwrap();
        fragments.set(id, serialized_data)
    }

    pub fn push(&mut self, id: String, data: Input) -> Result<(), SecureStoreError> {
        let serialized_data: Vec<u8> = Input::serialize(&[data])?; // Ensure correct type
        let mut fragments = self.fragments.lock().unwrap();
        fragments.push(id, serialized_data)
    }

    pub fn pop(&mut self, id: &str) -> Result<Option<Input>, SecureStoreError> {
        let mut fragments = self.fragments.lock().unwrap();
        if let Some(encrypted_data) = fragments.pop(id)? {
            let mut data: Vec<Input> = Input::deserialize(&encrypted_data)?;
            Ok(data.pop())
        } else {
            Ok(None)
        }
    }
}
