use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use rand_distr::WeightedIndex;
use rsa::rand_core::OsRng;

use crate::tools::config::Config;

/// Generates a random distribution of values.
///
/// ### Arguments
/// * `rng` - A random number generator.
/// * `range` - A range of values for the distribution.
/// * `size` - The number of values to generate.
///
/// ### Returns
/// A vector of randomly generated values.
pub fn create_random_distribution(
    rng: &mut StdRng,
    range: std::ops::RangeInclusive<u32>,
    size: usize,
) -> Vec<u32> {
    (0..size).map(|_| rng.gen_range(range.clone())).collect()
}

/// Randomly chooses an index from the provided vector based on the provided weights.
///
/// ### Arguments
/// * `weights` - A vector of weights. They are automatically normalized.
///
/// ### Returns
/// A random index from the provided weights, ensuring cryptographic security in the random choice process.
pub fn weighted_random_choice(weights: &[u32]) -> usize {
    // Initialize a cryptographically secure RNG
    let mut rng = create_seeded_rng();

    // Convert weights to f64 and normalize
    let total_weight: f64 = weights.iter().map(|&w| w as f64).sum();
    let normalized_weights: Vec<f64> = weights.iter().map(|&w| (w as f64) / total_weight).collect();

    // Use WeightedIndex for unbiased selection
    let dist = WeightedIndex::new(&normalized_weights)
        .expect("Weighted random choice: weight distribution creation failed");

    // Perform the weighted random choice
    dist.sample(&mut rng)
}

/// Generates a seeded RNG using hardware entropy and entropy-based seed.
/// Optionally, it incorporates a randomized Lorenz system.
///
/// ### Returns
/// A seeded RNG (`StdRng`).
pub fn create_seeded_rng() -> StdRng {
    let has_enhanced_seed_generation = Config::get_features().enhanced_seed_generation;

    let mut rng = StdRng::from_entropy();

    let mut combined_seed_bytes = generate_combined_seed(&mut rng);

    // Incorporate Lorenz system entropy if enhanced seed generation is enabled
    if has_enhanced_seed_generation {
        let initial_conditions = (rng.gen(), rng.gen(), rng.gen());
        let lorenz_entropy = generate_randomized_lorenz_entropy(&mut rng, 100, initial_conditions);
        enhance_seed_with_lorenz_entropy(&mut combined_seed_bytes, &lorenz_entropy);
    }

    // Final shuffle
    shuffle_seed(&mut combined_seed_bytes, &mut rng);

    StdRng::from_seed(combined_seed_bytes)
}

/// Generates a combined seed using hardware and entropy-based RNGs.
///
/// ### Arguments
/// * `rng` - A random number generator (`StdRng`).
///
/// ### Returns
/// A 32-byte array representing the combined seed.
fn generate_combined_seed(rng: &mut StdRng) -> [u8; 32] {
    let mut os_rng = OsRng;
    let mut os_seed_bytes = [0u8; 32];
    os_rng.fill_bytes(&mut os_seed_bytes);

    let mut entropy_seed_bytes = [0u8; 32];
    rng.fill_bytes(&mut entropy_seed_bytes);

    os_seed_bytes
        .iter()
        .zip(entropy_seed_bytes.iter())
        .map(|(&os_byte, &entropy_byte)| os_byte ^ entropy_byte)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|_| [0u8; 32]) // Fallback for conversion error
}

/// Enhances the seed with Lorenz system entropy.
///
/// ### Arguments
/// * `seed` - The seed to enhance.
/// * `lorenz_entropy` - Entropy generated from the Lorenz system.
fn enhance_seed_with_lorenz_entropy(seed: &mut [u8; 32], lorenz_entropy: &[u8]) {
    for (i, &byte) in lorenz_entropy.iter().enumerate() {
        seed[i % 32] ^= byte;
    }
}

/// Shuffles the seed using a uniform distribution.
///
/// ### Arguments
/// * `seed` - The seed to shuffle.
/// * `rng` - A random number generator (`StdRng`).
fn shuffle_seed(seed: &mut [u8; 32], rng: &mut StdRng) {
    let has_enhanced_seed_generation = Config::get_features().enhanced_seed_generation;
    let shuffle_range = if has_enhanced_seed_generation {
        Uniform::from(0..1000).sample(rng)
    } else {
        100
    };

    let uniform_dist = Uniform::from(0..32);
    for _ in 0..shuffle_range {
        let idx1 = uniform_dist.sample(rng) as usize;
        let idx2 = uniform_dist.sample(rng) as usize;
        seed.swap(idx1, idx2);
    }
}

/// Generates entropy using a randomized Lorenz system.
/// if enhanced seed generation is enabled, the Lorenz system parameters are *slightly* randomized.
///
/// ### Arguments
/// * `rng` - A random number generator for varying Lorenz system parameters.
/// * `steps` - Number of steps to simulate the Lorenz system.
/// * `initial_conditions` - Initial values for the Lorenz system variables (x, y, z).
///
/// ### Returns
/// A vector of bytes representing the entropy generated from the Lorenz system.
fn generate_randomized_lorenz_entropy(
    rng: &mut StdRng,
    steps: usize,
    initial_conditions: (f64, f64, f64),
) -> Vec<u8> {
    let (mut x, mut y, mut z) = initial_conditions;

    let has_enhanced_seed_generation = Config::get_features().enhanced_seed_generation;

    // Randomize parameters within a certain range if enhanced seed generation is enabled
    let sigma = if has_enhanced_seed_generation {
        rng.gen_range(8.0..12.0)
    } else {
        10.0
    };
    let rho = if has_enhanced_seed_generation {
        rng.gen_range(26.0..30.0)
    } else {
        28.0
    };
    let beta = if has_enhanced_seed_generation {
        rng.gen_range(2.0..4.0)
    } else {
        3.0
    };

    let dt = 0.01; // Time step for the simulation
    let mut entropy = Vec::with_capacity(steps * 3);

    for _ in 0..steps {
        let dx = sigma * (y - x) * dt;
        let dy = (x * (rho - z) - y) * dt;
        let dz = (x * y - beta * z) * dt;

        x += dx;
        y += dy;
        z += dz;

        // Convert Lorenz system state to bytes and add to entropy
        entropy.push((x.abs() % 256.0) as u8);
        entropy.push((y.abs() % 256.0) as u8);
        entropy.push((z.abs() % 256.0) as u8);
    }

    entropy
}
