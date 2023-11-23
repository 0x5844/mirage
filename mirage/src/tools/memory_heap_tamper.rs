use super::{Tool, Tools};
use crate::utils::math::statistics_probability::{
    create_random_distribution, create_seeded_rng, weighted_random_choice,
};
use crate::utils::sleep::sleep_random_duration;
use log::debug;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::Rng;
use rand_distr::num_traits::real::Real;
use rand_distr::Normal;
use std::ptr;

pub struct MemoryHeapTamper {
    rng: StdRng,
}

impl MemoryHeapTamper {
    pub fn new() -> Self {
        let rng = create_seeded_rng();
        if cfg!(feature = "development") {
            debug!("[Tools/{}] Initializing...", Tools::MemoryHeapTamper);
        }
        MemoryHeapTamper { rng }
    }

    fn tamper_memory(&mut self, block_size: usize) -> Vec<u8> {
        let pattern_distribution = create_random_distribution(&mut self.rng, 10..=100, 4);
        (0..block_size)
            .map(|i| {
                let pattern_choice = weighted_random_choice(&pattern_distribution);
                match pattern_choice {
                    0 => self.rng.gen::<u8>(),
                    1 => 0xFF,
                    2 => i as u8,
                    _ => Normal::new(150.0, 50.0).unwrap().sample(&mut self.rng) as u8,
                }
            })
            .collect()
    }

    pub unsafe fn tamper_memory_pointer(&mut self, memory_ptr: *mut u8, block_size: usize) {
        let tampered_memory = self.tamper_memory(block_size);
        for (i, &value) in tampered_memory.iter().enumerate() {
            if i < block_size {
                ptr::write_volatile(memory_ptr.add(i), value);
            }
        }
    }

    /// Tampers adjacent blocks of memory around the specified memory fragments.
    ///
    /// ### Arguments
    /// * `fragments` - A vector of tuples containing pointers to the fragments and their sizes.
    /// * `tamper_range` - The range around each fragment to apply tampering, in bytes.
    pub unsafe fn tamper_memory_fragments_adjacent(
        &mut self,
        fragments: &[(usize, usize)],
        tamper_range: usize,
    ) {
        for &(ptr, size) in fragments {
            // Calculate the safe start and end addresses for tampering
            let start_address = if ptr >= tamper_range {
                ptr - tamper_range
            } else {
                0
            };
            let end_address = ptr + size + tamper_range;

            // Tamper before the fragment
            if start_address < ptr {
                let tamper_size = ptr - start_address;
                self.tamper_memory_pointer(start_address as *mut u8, tamper_size);
            }

            // Tamper after the fragment
            if end_address > (ptr + size) {
                let tamper_size = end_address - (ptr + size);
                self.tamper_memory_pointer((ptr + size) as *mut u8, tamper_size);
            }
        }
    }
}

impl Tool for MemoryHeapTamper {
    fn name(&self) -> &Tools {
        &Tools::MemoryHeapTamper
    }

    fn start(&mut self) {
        let block_count = self.rng.gen_range(3..=15);
        for _ in 0..block_count {
            let random_mean = Uniform::new(10.0, 200.0).sample(&mut self.rng);
            let random_std_dev = Uniform::new(1.0, 50.0).sample(&mut self.rng);
            let normal_block_size = Normal::new(random_mean, random_std_dev).unwrap();

            let block_size = normal_block_size.sample(&mut self.rng).max(10_000.0) as usize;
            if cfg!(feature = "development") {
                debug!(
                    "[Tools/{}] Tampering {} bytes",
                    Tools::MemoryHeapTamper,
                    block_size
                );
            }

            unsafe {
                let heap_memory_ptr = vec![0u8; block_size].as_mut_ptr();
                self.tamper_memory_pointer(heap_memory_ptr, block_size);
            }

            if cfg!(feature = "development") {
                debug!("[Tools/{}] Sleeping 100-500 ms", Tools::MemoryHeapTamper);
            }
            sleep_random_duration(100, 500);
        }
    }
}
