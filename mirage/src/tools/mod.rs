use std::fmt;

pub mod config;
pub mod memory_heap_modifier;
pub mod memory_scramble;

pub enum Tools {
    MemoryHeapModifier,
    MemoryScramble,
    Config,
    SecureMemory,
}

impl fmt::Display for Tools {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tools::MemoryHeapModifier => write!(f, "memory-heap-modifier"),
            Tools::MemoryScramble => write!(f, "memory-scramble"),
            Tools::Config => write!(f, "config"),
            Tools::SecureMemory => write!(f, "secure-memory"),
        }
    }
}

pub trait Tool {
    fn name(&self) -> &Tools;
    fn start(&mut self);
}
