mod actors;
mod tools;
mod utils;

use crate::actors::memory::Input;
use actors::memory::secure_memory_provider::SecureMemoryProvider;
use std::sync::{Arc, Mutex};
use tools::{config::Config, Tool};

#[tokio::main]
async fn main() {
    // Initialize the logger
    utils::logger::initialize();

    // Initialize and execute ConfigTool to load the configuration
    let mut config_tool = Config::new();
    config_tool.start();

    // Create a SecureMemoryProvider instance
    // Ensure that SecureMemoryProvider's new() function is compatible with async context
    let secure_memory_provider = Arc::new(Mutex::new(SecureMemoryProvider::new()));

    // Encrypt and store a string
    let sample_data = Input::Buffer("hello".as_bytes().to_vec());
    let sample_id = "sample_id";
    {
        let mut provider = secure_memory_provider.lock().unwrap();
        provider.push(sample_id.to_owned(), sample_data);
    }

    // Retrieve and decrypt the string
    {
        let provider = secure_memory_provider.lock().unwrap();
        if let Ok(Some(data)) = provider.get(sample_id) {
            match data.first() {
                Some(Input::Buffer(decrypted_data)) => {
                    println!(
                        "Decrypted data: {}",
                        String::from_utf8_lossy(&decrypted_data)
                    );
                }
                _ => println!("Unexpected data format"),
            }
        } else {
            println!("Error retrieving data");
        }
    }
}
