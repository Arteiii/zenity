use std::env;
use std::io::{stdout, Write};

use chrono::Local;
use crossterm::queue;
use crossterm::style::{Attribute, ContentStyle};
use log::{Level, Metadata, Record, SetLoggerError};

use crate::style::combine_attributes;
use crate::{color::ENABLE_COLOR, style, style::Color};

// Define a custom logger
struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // This logger will be enabled for all levels based on the configured log level
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        // Customize the log output format here
        let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level_str = format!("{}", record.level());
        let padded_level_str = format!("{:^5}", level_str); // Center-align the level string within a width of 5
        let mut stdout = stdout();

        queue!(
            stdout,
            style::SetStyle(ContentStyle {
                foreground_color: Some(Color::DarkGrey),
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Italic]),
            }),
            style::Print(format!("{}  ", current_time)),
            style::ResetColor, // reset colors
        )
        .unwrap();

        // Set foreground color based on log level
        let level_color = match record.level() {
            Level::Error => ContentStyle {
                foreground_color: Some(Color::Red),
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Bold]),
            },
            Level::Warn => ContentStyle {
                foreground_color: Some(Color::Yellow),
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Bold]),
            },
            Level::Info => ContentStyle {
                foreground_color: Some(Color::Green),
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Bold]),
            },
            Level::Debug => ContentStyle {
                foreground_color: Some(Color::Magenta),
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Bold]),
            },
            Level::Trace => ContentStyle {
                foreground_color: Some(Color::White),
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Bold]),
            },
        };

        if *ENABLE_COLOR {
            queue!(
                stdout,
                style::SetStyle(level_color), // set animation color
            )
            .unwrap();
        }

        queue!(
            stdout,
            style::Print(&padded_level_str),
            style::ResetColor, // reset colors
        )
        .unwrap();

        queue!(
            stdout,
            style::SetStyle(ContentStyle {
                foreground_color: Some(Color::DarkGrey),
                background_color: None,
                underline_color: None,
                attributes: combine_attributes(&[&Attribute::Italic]),
            }),
            style::Print(format!(" ({}):", record.target())),
            style::ResetColor, // reset colors
        )
        .unwrap();

        queue!(
            stdout,
            style::Print(format!(" {}", record.args())),
            style::ResetColor, // reset colors
        )
        .unwrap();

        queue!(stdout, style::Print("\n")).unwrap();

        stdout.flush().unwrap();
    }

    fn flush(&self) {}
}

/// Implement a builder pattern for initializing the logger
pub struct Logger {
    log_level: Level,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    /// Constructs a new `Logger` with default log level `Info`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::log::Logger;
    ///
    /// let logger = Logger::new();
    /// ```
    pub fn new() -> Logger {
        Logger {
            log_level: Level::Info,
        }
    }

    /// Initializes the logger with the custom formatter and log level based on environment variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::log::Logger;
    ///
    /// let logger = Logger::new().with_env("LOG_LEVEL").init().unwrap();
    /// ```
    ///
    /// # Notes
    ///
    /// This method sets the logger and the log level based on the environment variable or defaults to `Info`.
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_logger(&MyLogger)?;
        log::set_max_level(self.log_level.to_level_filter());
        Ok(())
    }

    /// Sets log level based on the environment variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::log::Logger;
    ///
    /// let logger = Logger::new().with_env("LOG_LEVEL");
    /// ```
    ///
    /// # Notes
    ///
    /// This method checks the specified environment variable for the log level and updates if it's lower than the current level.
    pub fn with_env(mut self, env_var: &str) -> Logger {
        if let Ok(log_level) = env::var(env_var) {
            if let Ok(level) = log_level.parse::<Level>() {
                if level < self.log_level {
                    self.log_level = level;
                }
            }
        }
        self
    }

    /// Sets log level based on the command-line argument if provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::log::Logger;
    ///
    /// let logger = Logger::new().with_arg();
    /// ```
    ///
    /// # Notes
    ///
    /// This method checks command-line arguments for the `--log-level=` option and updates the log level if provided and higher than the current level.
    pub fn with_arg(mut self) -> Logger {
        // Get command-line arguments
        let args: Vec<String> = env::args().collect();

        // Check if the "--log-level=" option is provided in the arguments
        if let Some(arg) = args.iter().find(|&arg| arg.starts_with("--log-level=")) {
            // Extract the log level from the argument
            if let Some(level_str) = arg.split('=').nth(1) {
                if let Ok(level) = level_str.parse::<Level>() {
                    // Prefer the higher log level between the argument and the one from the environment variable
                    if level < self.log_level {
                        self.log_level = level;
                    }
                }
            }
        }
        self
    }

    /// Sets log level. This overwrites the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::log::{Logger, Level};
    ///
    /// let logger = Logger::new().set_log_level(Level::Debug);
    /// ```
    ///
    /// # Notes
    ///
    /// This method directly sets the log level, overwriting the current value.
    pub fn set_log_level(mut self, level: Level) -> Logger {
        self.log_level = level;

        self
    }
}
