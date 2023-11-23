// use crate::tools::Tools;
// use crate::utils::math::statistics_probability::{
//     create_random_distribution, create_seeded_rng, weighted_random_choice,
// };
// use crate::utils::sleep::sleep_random_duration;
// use log::debug;
// use rand::distributions::{Distribution, Uniform};
// use rand::rngs::StdRng;
// use rand::seq::SliceRandom;
// use rand::Rng;
// use rand_distr::num_traits::real::Real;
// use rand_distr::Normal;
// use std::ptr;

// use super::config::Config;
// use super::Tool;

// /// Represents a tool for modifying memory heap contents in a controlled manner.
// pub struct MemoryHeapModifier {
//     rng: StdRng,
// }

// impl MemoryHeapModifier {
//     /// Creates a new memory heap modifier with a seeded random number generator.
//     pub fn new() -> Self {
//         let rng = create_seeded_rng();
//         if cfg!(feature = "development") {
//             debug!("[Tools/{}] Initializing...", Tools::MemoryHeapModifier);
//         }
//         MemoryHeapModifier { rng }
//     }

//     /// Generates a tampered memory pattern based on the specified block size.
//     fn generate_tampered_pattern(&mut self, block_size: usize) -> Vec<u8> {
//         let pattern_distribution: Vec<u32> = create_random_distribution(&mut self.rng, 10..=100, 4);
//         (0..block_size)
//             .map(|i| self.select_tamper_value(i, &pattern_distribution))
//             .collect()
//     }

//     /// Selects a value for tampering based on the current index and pattern distribution.
//     ///
//     /// # Arguments
//     /// * `index` - The current index in the memory block.
//     /// * `pattern_distribution` - The distribution pattern for tampering, adjusted to the correct type.
//     fn select_tamper_value(&mut self, index: usize, pattern_distribution: &[u32]) -> u8 {
//         let pattern_choice = weighted_random_choice(pattern_distribution);
//         let has_enhanced_heap_modification = Config::get_features().enhanced_heap_modification;

//         if has_enhanced_heap_modification {
//             pattern_distribution.shuffle(&mut self.rng);
//         }

//         let mean = if has_enhanced_heap_modification {
//             Uniform::new(10.0, 200.0).sample(&mut self.rng)
//         } else {
//             150.0
//         };

//         let std_dev = if has_enhanced_heap_modification {
//             Uniform::new(1.0, 50.0).sample(&mut self.rng)
//         } else {
//             50.0
//         };

//         match pattern_choice {
//             0 => self.rng.gen::<u8>(),
//             1 => 0xFF,
//             2 => index as u8,
//             // 3 if has_enhanced_heap_modification => advanced_tampering_logic(&mut self.rng),
//             _ => Normal::new(mean, std_dev).unwrap().sample(&mut self.rng) as u8,
//         }
//     }

//     /// Advanced tampering logic that considers the index and a random distribution.
//     // fn advanced_tampering_logic(rng: &mut StdRng) -> u8 {}

//     /// Performs memory tampering at the specified pointer with the given block size.
//     ///
//     /// # Safety
//     /// This function performs raw pointer operations and should be used with caution.
//     pub unsafe fn tamper_at_pointer(&mut self, memory_ptr: *mut u8, block_size: usize) {
//         let tampered_memory = self.generate_tampered_pattern(block_size);
//         for (i, &value) in tampered_memory.iter().enumerate() {
//             if i < block_size {
//                 ptr::write_volatile(memory_ptr.add(i), value);
//             }
//         }
//     }

//     /// Tampers with memory adjacent to specified fragments.
//     ///
//     /// # Arguments
//     /// * `fragments` - Tuples of memory fragment pointers and their sizes.
//     /// * `tamper_range` - Range around each fragment to apply tampering.
//     pub unsafe fn tamper_adjacent_fragments(
//         &mut self,
//         fragments: &[(usize, usize)],
//         tamper_range: usize,
//     ) {
//         for &(ptr, size) in fragments {
//             let (start, end) = self.calculate_tamper_bounds(ptr, size, tamper_range);

//             if start < ptr {
//                 self.tamper_at_pointer(start as *mut u8, ptr - start);
//             }

//             if end > ptr + size {
//                 self.tamper_at_pointer((ptr + size) as *mut u8, end - (ptr + size));
//             }
//         }
//     }

//     /// Calculates the safe start and end addresses for tampering.
//     fn calculate_tamper_bounds(
//         &self,
//         ptr: usize,
//         size: usize,
//         tamper_range: usize,
//     ) -> (usize, usize) {
//         let start_address = ptr.saturating_sub(tamper_range);
//         let end_address = ptr.saturating_add(size).saturating_add(tamper_range);
//         (start_address, end_address)
//     }

//     /// Starts the tampering process based on random block sizes.
//     fn start_tampering(&mut self) {
//         let block_count = self.rng.gen_range(3..=15);
//         for _ in 0..block_count {
//             let random_mean = Uniform::new(10.0, 200.0).sample(&mut self.rng);
//             let random_std_dev = Uniform::new(1.0, 50.0).sample(&mut self.rng);
//             let normal_block_size = Normal::new(random_mean, random_std_dev).unwrap();

//             let block_size = normal_block_size.sample(&mut self.rng).max(10_000.0) as usize;
//             if cfg!(feature = "development") {
//                 debug!(
//                     "[Tools/{}] Tampering {} bytes",
//                     Tools::MemoryHeapModifier,
//                     block_size
//                 );
//             }

//             unsafe {
//                 let heap_memory_ptr = vec![0u8; block_size].as_mut_ptr();
//                 self.tamper_at_pointer(heap_memory_ptr, block_size);
//             }

//             sleep_random_duration(100, 500);
//         }
//     }
// }

// impl Tool for MemoryHeapModifier {
//     fn name(&self) -> &Tools {
//         &Tools::MemoryHeapModifier
//     }

//     fn start(&mut self) {
//         self.start_tampering();
//     }
// }
