use std::alloc::{self, Layout};

use aes::cipher::generic_array::GenericArray;
use aes_gcm::{
    aead::{Aead, Payload},
    Aes128Gcm, KeyInit,
};
use log::error;
use rand::{rngs::StdRng, Rng};
use rand_distr::{Distribution, Normal};

use super::math::statistics_probability::create_random_distribution;

/// Utility function to create a memory layout based on input size in bytes.
pub fn create_layout_from_size(block_size: usize) -> Layout {
    let align = std::mem::align_of::<u32>();
    Layout::from_size_align(block_size * std::mem::size_of::<u32>(), align)
        .expect("Failed to create memory layout")
}

/// Utility function to allocate a memory buffer based on `Layout`.
pub unsafe fn allocate_memory_from_layout(layout: Layout) -> *mut u32 {
    let heap_memory = alloc::alloc(layout) as *mut u32;
    if heap_memory.is_null() {
        panic!("Failed to allocate memory");
    }
    heap_memory
}

// Utility function to allocate a memory buffer based on input size in bytes.
pub fn allocate_memory_from_size(size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, 4).expect("Failed to create memory layout");
        let heap_memory = alloc::alloc(layout) as *mut u8;
        if heap_memory.is_null() {
            panic!("Failed to allocate memory");
        }
        heap_memory
    }
}

/// Utility function to shuffle a memory buffer in place using `StdRng`.
pub fn shuffle_buffer_with_rng(buffer: &mut [u8], rng: &mut StdRng) {
    let len = buffer.len();
    let distribution = create_random_distribution(rng, 0..=len as u32 - 1, len);
    let mut shuffled_buffer = vec![0u8; len];
    distribution.iter().enumerate().for_each(|(i, &index)| {
        shuffled_buffer[i] = buffer[index as usize % len];
    });
    buffer.copy_from_slice(&shuffled_buffer);
}

/// Utility function to XOR a memory buffer in place with a random seed.
pub fn xor_buffer_with_seed(buffer: &mut [u8], rng: &mut StdRng) {
    let seed = rng.gen::<u8>();
    buffer.iter_mut().for_each(|byte| *byte ^= seed);
}

/// Utility function to encrypt memory buffer in place AES-128-GCM using `Key` and `Nonce`.
pub fn encrypt_buffer_aes(buffer: &[u8], rng: &mut StdRng) -> Vec<u8> {
    // Create variables to hold the key and nonce values
    let key_bytes = rng.gen::<[u8; 16]>();
    let nonce_bytes = rng.gen::<[u8; 12]>();

    // Use the created variables to initialize key and nonce
    let key = GenericArray::from_slice(&key_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);
    let cipher = Aes128Gcm::new(key);

    let payload = Payload {
        msg: buffer,
        aad: b"".as_ref(),
    };

    match cipher.encrypt(nonce, payload) {
        Ok(encrypted_data) => encrypted_data,
        Err(e) => {
            error!("AES-GCM Encryption failed: {}", e);
            vec![] // Handle error appropriately
        }
    }
}

/// Utility function to apply Gaussian noise to a memory buffer in place.
pub fn apply_gaussian_noise_on_buffer(buffer: &mut [u8], mut rng: &mut StdRng) {
    let mean = rng.gen::<f32>();
    let std_dev = rng.gen::<f32>();
    let gaussian_distribution = Normal::new(mean, std_dev).unwrap();
    buffer.iter_mut().for_each(|byte| {
        let noise = gaussian_distribution.sample(&mut rng).round() as i8;
        *byte = byte.wrapping_add(noise as u8);
    });
}
