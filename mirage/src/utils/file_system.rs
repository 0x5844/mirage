use rand::{rngs::OsRng, Rng, RngCore};
use rand_distr::StandardNormal;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

pub fn secure_delete_file(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let metadata = fs::metadata(&path)?;
    let length = metadata.len();
    let mut file = File::options().write(true).open(&path)?;

    // Overwrite patterns
    overwrite_file_byte(&mut file, length, 0x00)?;
    overwrite_file_byte(&mut file, length, 0xFF)?;

    // Third pass: write Gaussian noise
    overwrite_file_gaussian_noise(&mut file, length)?;

    // Forth pass: perform bitwise XOR operation with a random byte
    overwrite_file_xor(&mut file, length)?;

    // Truncate and remove the file
    file.set_len(0)?;
    drop(file); // Close the file before deletion
    fs::remove_file(&path)
}

fn overwrite_file_byte(file: &mut File, length: u64, byte: u8) -> io::Result<()> {
    let buffer = vec![byte; length as usize];
    file.write_all(&buffer)?;
    file.sync_all()
}

fn overwrite_file_gaussian_noise(file: &mut File, length: u64) -> io::Result<()> {
    let mut rng = OsRng;
    let normal_dist = StandardNormal;
    let mut buffer = vec![0; length as usize];
    for byte in buffer.iter_mut() {
        let noise: f64 = rng.sample(normal_dist); // Gaussian noise
        *byte = noise.abs() as u8;
    }
    file.write_all(&buffer)?;
    file.sync_all()
}

fn overwrite_file_xor(file: &mut File, length: u64) -> io::Result<()> {
    let mut rng = OsRng;
    let mut buffer = vec![0; length as usize];
    rng.fill_bytes(&mut buffer); // Generate random bytes
    let random_byte = rng.gen::<u8>(); // Generate a random byte
    for byte in buffer.iter_mut() {
        *byte ^= random_byte; // Perform bitwise XOR operation
    }
    file.write_all(&buffer)?;
    file.sync_all()
}
