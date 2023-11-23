use aes::cipher::{generic_array::GenericArray, typenum::U12};
use aes_gcm::{aead::Aead, Aes128Gcm, Aes256Gcm, KeyInit, Nonce};
use log::{debug, error};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    actors::Actor,
    tools::config::{Config, EncryptionLevel},
};

use super::{AesError, CryptoBase, Input, KEY_LENGTH};

#[derive(Clone)]
pub struct Encryptor {
    base: CryptoBase,
}

impl Actor<Input> for Encryptor {
    fn is_alive(&self) -> bool {
        self.base.is_alive
    }

    fn run(&self, input: Option<Input>) {
        let runtime_handle = Arc::clone(&self.base.runtime_handle);

        if let Some(input_type) = input {
            match input_type {
                Input::File(path) => {
                    let encryptor_clone = self.clone();
                    runtime_handle.spawn(async move {
                        if let Err(e) = encryptor_clone.process_file(path).await {
                            error!("Error processing file: {:?}", e);
                        }
                    });
                }
                Input::Buffer(mut data) => {
                    let encryptor_clone = self.clone();
                    runtime_handle.spawn(async move {
                        if let Err(e) = encryptor_clone.process_buffer_or_bit(&mut data).await {
                            error!("Error processing buffer: {:?}", e);
                        }
                    });
                }
                Input::Bit(bit) => {
                    let mut data = vec![bit];
                    let encryptor_clone = self.clone();
                    runtime_handle.spawn(async move {
                        if let Err(e) = encryptor_clone.process_buffer_or_bit(&mut data).await {
                            error!("Error processing bit: {:?}", e);
                        }
                    });
                }
            }
        }
    }
}

impl Encryptor {
    pub fn new(
        level: Option<EncryptionLevel>,
        key: Vec<u8>,
        nonce: Vec<u8>,
        runtime_handle: Arc<tokio::runtime::Handle>,
    ) -> Self {
        let level = level.unwrap_or_else(|| Config::get_parameters().encryption_level);
        let has_parallel_processing = Config::get_features().parallel_processing;

        Encryptor {
            base: CryptoBase {
                is_alive: true,
                level,
                key: Arc::new(key),
                nonce: Arc::new(nonce),
                has_parallel_processing,
                runtime_handle,
            },
        }
    }

    async fn process_file(&self, path: PathBuf) -> Result<(), AesError> {
        let mut file = File::open(&path).await.map_err(AesError::IoError)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .map_err(AesError::IoError)?;

        let encrypted_data = self.encrypt(&buffer)?;

        let mut file = File::create(&path).await.map_err(AesError::IoError)?;
        file.write_all(&encrypted_data)
            .await
            .map_err(AesError::IoError)?;

        Ok(())
    }

    async fn process_buffer_or_bit(&self, buffer: &mut Vec<u8>) -> Result<(), AesError> {
        let encrypted_data = self.encrypt(buffer)?;

        buffer.clear();
        buffer.extend_from_slice(&encrypted_data);

        Ok(())
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, AesError> {
        match &self.base.level {
            EncryptionLevel::Level1 => self.encrypt_aes_gcm::<Aes128Gcm>(data),
            EncryptionLevel::Level2 => self.encrypt_aes_gcm::<Aes256Gcm>(data),
        }
    }

    fn encrypt_aes_gcm<C>(&self, data: &[u8]) -> Result<Vec<u8>, AesError>
    where
        C: Aead + KeyInit + Send + Sync + 'static,
    {
        if self.base.nonce.len() != KEY_LENGTH {
            return Err(AesError::InvalidKeyLength);
        }

        let cipher = Arc::new(C::new_from_slice(&self.base.key).map_err(AesError::from)?);

        let nonce_generic = GenericArray::<u8, U12>::from_slice(&self.base.nonce);
        let nonce = Nonce::from_slice(nonce_generic);

        if cfg!(feature = "development") {
            debug!(
                "[Encryptor] Encrypting data with \nkey: {:?} \nnonce: {:?}",
                self.base.key, self.base.nonce
            );
        }

        if self.base.has_parallel_processing {
            data.par_chunks(1024)
                .map_init(
                    || cipher.clone(),
                    |cipher, chunk| cipher.encrypt(nonce, chunk).map_err(AesError::from),
                )
                .collect::<Result<Vec<_>, _>>()
                .map(|chunks| chunks.concat())
        } else {
            cipher.encrypt(nonce, data).map_err(AesError::from)
        }
    }
}
