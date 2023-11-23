use rand::Rng;

use super::math::statistics_probability::create_seeded_rng;

/// Utility function that sleeps for a random duration within the specified range.
///
/// ### Arguments
/// * `rng` - A mutable reference to a StdRng random number generator.
/// * `start_time` - The start of the range in milliseconds for the random sleep duration.
/// * `end_time` - The end of the range in milliseconds for the random sleep duration.
pub fn sleep_random_duration(start_time: u64, end_time: u64) {
    let mut rng = create_seeded_rng();
    let sleep_duration = rng.gen_range(start_time..=end_time);
    std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
}
