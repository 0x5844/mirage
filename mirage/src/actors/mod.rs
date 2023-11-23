pub mod encryption;
pub mod evasion;
pub mod memory;

pub trait Actor<T: Send + Sync>: Send {
    fn is_alive(&self) -> bool;
    fn run(&self, input: Option<T>);
}
