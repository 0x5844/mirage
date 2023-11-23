use rand::Rng;

use crate::tools::config::{Config, EncryptionLevel};

use super::math::statistics_probability::create_seeded_rng;

const NONCE_SIZE: usize = 12;

pub fn generate_aes_key() -> Vec<u8> {
    let level = Config::get_parameters().encryption_level;
    let key_size = match level {
        EncryptionLevel::Level1 => 16, // AES128 (16 bytes)
        EncryptionLevel::Level2 => 32, // AES256 (32 bytes)
    };

    let mut rng = create_seeded_rng();
    (0..key_size).map(|_| rng.gen::<u8>()).collect()
}

pub fn generate_nonce() -> Vec<u8> {
    let mut rng = create_seeded_rng();
    (0..NONCE_SIZE).map(|_| rng.gen::<u8>()).collect()
}
