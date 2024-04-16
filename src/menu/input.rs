//! Input Validation Widgets
//!
//! This module provides functions for validating user input with various criteria, such as regex patterns or file paths
//! The functions prompt the user for input, validate it, and return the validated input
//!
//! # Examples
//!
//! ## Validate Input Against Regex
//!
//! ```rust
//! use regex::Regex;
//! use zenity::menu::input::valid_regex;
//!
//! // Define a regex pattern to match three digits
//! let regex = Regex::new(r"^\d{3}$").unwrap();
//!
//! // Prompt the user to enter input matching the regex pattern
//! let input = valid_regex(regex);
//!
//! println!("Valid input: {}", input);
//! ```
//!
//! ## Validate File Path
//!
//! ```rust
//! use std::path::PathBuf;
//! use zenity::menu::input::valid_path;
//!
//! // Prompt the user to enter a valid file path
//! let path: PathBuf = valid_path().into();
//!
//! println!("Entered path: {:?}", path);
//! ```
//!
//! This module is a work in progress, and contributions are welcome
//!

use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent},
    ExecutableCommand,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use regex::Regex;

use crate::style::{Color, Print, SetForegroundColor};

/// Validates and returns a string that matches the specified regex pattern.
///
/// This function prompts the user to enter input and validates the input against the provided
/// regex pattern. It continues to prompt the user until the entered input matches the regex pattern.
/// The function returns the validated input as a string.
///
/// # Arguments
///
/// * `regex` - A `Regex` object representing the regex pattern to match against the user input.
///
/// # Returns
///
/// A `String` containing the user input that matches the specified regex pattern.
///
/// # Example
///
/// ```
/// use regex::Regex;
/// use zenity::menu::input::valid_regex;
///
/// // Define a regex pattern to match three digits
/// let regex = Regex::new(r"^\d{3}$").unwrap();
///
/// // Prompt the user to enter input matching the regex pattern
/// let input = valid_regex(regex);
///
/// println!("Valid input: {}", input);
/// ```
pub fn valid_regex(regex: Regex) -> String {
    enable_raw_mode().expect("Failed to enable raw-mode");

    let mut stdout = stdout();
    let mut buffer = String::new();

    // fix for windows double input
    let mut skip_next = false;

    loop {
        if handle_key_input(&mut buffer, &mut skip_next)
            && validate_input(&buffer, &regex, &mut stdout)
        {
            break;
        }

        execute!(
            stdout,
            MoveTo(0, 0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
            Print("Enter input: "),
            if !regex.is_match(&buffer) {
                SetForegroundColor(Color::DarkRed)
            } else {
                SetForegroundColor(Color::Green)
            },
            Print(&buffer),
            SetForegroundColor(Color::Reset)
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    disable_raw_mode().expect("Failed to disable raw-mode");

    buffer
}

/// Validates and returns a `PathBuf` representing the entered path.
///
/// This function prompts the user to enter a path and validates the input. If the entered path is valid,
/// it returns a `PathBuf` containing the path. Otherwise, it continues prompting the user until a valid
/// path is entered.
///
/// # Examples
///
/// ```
/// use zenity::menu::input::valid_path;
///
/// let path = valid_path();
/// println!("Entered path: {:?}", path);
/// ```
pub fn valid_path() -> Box<PathBuf> {
    enable_raw_mode().expect("Failed to enable raw-mode");

    let mut stdout = stdout();
    let mut buffer = String::new();

    // to prevent double inputs on windows
    let mut skip_next = false;

    loop {
        if handle_key_input(&mut buffer, &mut skip_next) && validate_path(&buffer) {
            break;
        }

        execute!(
            stdout,
            MoveTo(0, 0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
            Print("Enter path: "),
            if !validate_path(&buffer) {
                SetForegroundColor(Color::DarkRed)
            } else {
                SetForegroundColor(Color::Green)
            },
            Print(&buffer),
            SetForegroundColor(Color::Reset)
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    disable_raw_mode().expect("Failed to disable raw-mode");

    let path = Path::new(&buffer).to_owned();

    Box::new(path)
}

fn validate_path(path: &str) -> bool {
    // useless function but might change something here later...
    Path::new(path).exists()
}

fn validate_input(buffer: &str, regex: &Regex, stdout: &mut std::io::Stdout) -> bool {
    if regex.is_match(buffer) {
        true
    } else {
        stdout
            .execute(MoveTo(0, 0))
            .unwrap()
            .execute(crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine,
            ))
            .unwrap();
        false 
    }
}

fn handle_key_input(buffer: &mut String, skip_next: &mut bool) -> bool {
    let event = crossterm::event::read().unwrap();

    // Check if we need to skip this key event
    if *skip_next {
        *skip_next = false; // Toggle the flag back
        return false;
    }

    // Handle events
    if let Event::Key(key_event) = event {
        let KeyEvent { code, .. } = key_event;

        match code {
            KeyCode::Enter => {
                return true; // signal to validate the input
            }
            KeyCode::Backspace => {
                buffer.pop();
            }
            KeyCode::Char(c) => {
                buffer.push(c);
            }
            _ => {
                return false; // Continue processing key events
            }
        }

        // to fix double inputs on windows
        #[cfg(windows)]
        {
            *skip_next = true;
        }
    }

    false
}
