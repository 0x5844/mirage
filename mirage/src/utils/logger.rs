use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn initialize() {
    // Initialize and configure the logger
    Builder::new()
        .format(|buf, record| {
            let level = record.level();
            // Format the log message based on the log level
            let message = match level {
                log::Level::Info => format!("ℹ {}", record.args()).bright_black(),
                log::Level::Trace => format!("⌕ {}", record.args()).cyan().italic(),
                log::Level::Error => format!("✖ {}", record.args()).red().bold(),
                log::Level::Debug => format!("𓆣ִ {}", record.args()).blue(),
                log::Level::Warn => format!("⚠️ {}", record.args()).yellow(),
            };
            // Write the formatted message to the buffer
            writeln!(buf, "{}", message)
        })
        .filter(None, LevelFilter::Debug)
        .init();
}
