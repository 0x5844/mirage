use log::debug;
use rand::rngs::StdRng;
use std::sync::{Arc, Mutex};

use super::config::Config;
use super::{Tool, Tools};
use crate::utils::math::statistics_probability::create_seeded_rng;
use crate::utils::memory::{
    apply_gaussian_noise_on_buffer, encrypt_buffer_aes, shuffle_buffer_with_rng,
    xor_buffer_with_seed,
};

const MB: usize = 1024 * 1024;

pub struct MemoryScramble {
    stealth_mode: Arc<Mutex<bool>>,
    rng: StdRng,
    memory_size: usize,
}

impl MemoryScramble {
    pub fn new() -> Self {
        let rng = create_seeded_rng();
        if cfg!(feature = "development") {
            debug!("[Tools/{}] Initializing...", Tools::MemoryScramble);
        }

        let is_stealth = Config::get_features().stealth;
        let configured_memory_size = Config::get_parameters().memory_scramble_size;
        let memory_size = configured_memory_size * MB;

        MemoryScramble {
            stealth_mode: Arc::new(Mutex::new(is_stealth)),
            rng,
            memory_size: if is_stealth {
                memory_size / 2
            } else {
                memory_size
            },
        }
    }

    fn adjust_memory_size(&mut self) {
        let is_stealth = *self.stealth_mode.lock().unwrap();
        let configured_memory_size = Config::get_parameters().memory_scramble_size * MB;
        self.memory_size = if is_stealth {
            configured_memory_size / 2
        } else {
            configured_memory_size
        };
        if cfg!(feature = "development") {
            debug!("[Tools/{}] Adjusting memory size", Tools::MemoryScramble);
            debug!(
                "[Tools/{}] Stealth mode: {}",
                Tools::MemoryScramble,
                is_stealth
            );
            debug!(
                "[Tools/{}] Memory size: {}",
                Tools::MemoryScramble,
                self.memory_size
            );
        }
    }

    /// Scrambles memory at specified fragments.
    ///
    /// ### Arguments
    /// * `fragments` - A vector of tuples containing pointers to the fragments and their sizes.
    pub fn scramble_memory_fragments(&mut self, fragments: &[(*mut u8, usize)]) {
        for &(ptr, size) in fragments {
            unsafe {
                // Check if the pointer is not null and the size is greater than 0
                if !ptr.is_null() && size > 0 {
                    let fragment_slice = std::slice::from_raw_parts_mut(ptr, size);

                    // Apply scrambling methods
                    xor_buffer_with_seed(fragment_slice, &mut self.rng);
                    shuffle_buffer_with_rng(fragment_slice, &mut self.rng);

                    // Apply stealth measures if enabled
                    if *self.stealth_mode.lock().unwrap() {
                        apply_gaussian_noise_on_buffer(fragment_slice, &mut self.rng);
                        encrypt_buffer_aes(fragment_slice, &mut self.rng);
                    }
                }
            }
        }
    }
}

impl Tool for MemoryScramble {
    fn name(&self) -> &Tools {
        &Tools::MemoryScramble
    }

    fn start(&mut self) {
        let mut buffer = vec![0u8; self.memory_size];
        let is_stealth = *self.stealth_mode.lock().unwrap();

        self.apply_basic_measures(&mut buffer);

        if is_stealth {
            self.apply_stealth_measures(&mut buffer);
        }
    }
}

impl MemoryScramble {
    fn apply_basic_measures(&mut self, memory_buffer: &mut [u8]) {
        xor_buffer_with_seed(memory_buffer, &mut self.rng);
        shuffle_buffer_with_rng(memory_buffer, &mut self.rng);
        if cfg!(feature = "development") {
            debug!("[Tools/{}] Applied Basic Measures", Tools::MemoryScramble);
        }
    }

    fn apply_stealth_measures(&mut self, memory_buffer: &mut [u8]) {
        apply_gaussian_noise_on_buffer(memory_buffer, &mut self.rng);
        encrypt_buffer_aes(memory_buffer, &mut self.rng);
        if cfg!(feature = "development") {
            debug!("[Tools/{}] Applied Stealth Measures", Tools::MemoryScramble);
        }
    }
}
