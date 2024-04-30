use log::Level;

use zenity::log::Logger;

fn main() {
    // Set the custom logger as the global logger
    Logger::new()
        .with_env("TEST_LEVEL")
        .with_arg()
        .set_log_level(Level::Trace)
        .init()
        .unwrap();

    // Now you can use log::debug! to output messages
    log::error!("This is a error message");
    log::warn!("This is a warn message");
    log::info!("This is a info message");
    log::debug!("This is a debug message");
    log::trace!("This is a trace message");
}
