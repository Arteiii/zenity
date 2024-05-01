use log::Level;

use zenity::log::Logger;

mod foo {
    mod bar {
        pub fn run() {
            log::error!("[bar] error");
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
            log::trace!("[bar] trace");
        }
    }

    pub fn run() {
        log::error!("[foo] error");
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        log::trace!("[foo] trace");
        bar::run();
    }
}

fn main() {
    // Set the custom logger as the global logger
    Logger::new()
        .with_env("TEST_LEVEL")
        .with_arg()
        .set_log_level(Level::Trace)
        .init()
        .unwrap();

    // Now you can use log::debug! to output messages
    log::error!("[root] This is a error message");
    log::warn!("[root] This is a warn message");
    log::info!("[root] This is a info message");
    log::debug!("[root] This is a debug message");
    log::trace!("[root] This is a trace message");
    foo::run();
}
