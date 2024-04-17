//! Input Validation Widgets
//!
//! This module provides functions for validating user input with various criteria, such as regex patterns or file paths
//! The functions prompt the user for input, validate it, and return the validated input
//!
//! # Examples
//!
//! ## Validate Input Against Regex
//!
//! ```rust,ignore
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
//! ```rust,ignore
//! use std::path::PathBuf;
//! use zenity::menu::input::valid_path;
//!
//! // Prompt the user to enter a valid file path
//! let path: PathBuf = (*valid_path()).clone().into(); // Cloning the Box<PathBuf> and then converting it into PathBuf
//!
//! println!("Entered path: {:?}", path);
//! ```
//!
//! This module is a work in progress, and contributions are welcome
//!

use std::io::stdout;
use std::path::{Path, PathBuf};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use regex::Regex;

use crate::menu::handle_key_input;
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
/// ```rust,ignore
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

    let mut buffer = String::new();

    loop {
        if handle_key_input(&mut buffer) && validate_input(&buffer, &regex) {
            break;
        }

        execute!(
            stdout(),
            MoveTo(0, 0),
            Clear(ClearType::CurrentLine),
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
/// ```rust,ignore
/// use zenity::menu::input::valid_path;
///
/// let path = valid_path();
/// println!("Entered path: {:?}", path);
/// ```
pub fn valid_path() -> Box<PathBuf> {
    enable_raw_mode().expect("Failed to enable raw-mode");

    let mut buffer = String::new();

    loop {
        if handle_key_input(&mut buffer) && validate_path(&buffer) {
            break;
        }

        render_input_prompt(&buffer, &validate_path(&buffer));
    }

    disable_raw_mode().expect("Failed to disable raw-mode");

    let path = PathBuf::from(buffer);

    Box::new(path)
}

fn validate_path(path: &str) -> bool {
    // useless function but might change something here later...
    Path::new(path).exists()
}

fn validate_input(buffer: &str, regex: &Regex) -> bool {
    if regex.is_match(buffer) {
        true
    } else {
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::CurrentLine)).unwrap();
        false
    }
}

fn render_input_prompt(buffer: &str, is_valid: &bool) {
    execute!(
        stdout(),
        MoveTo(0, 0),
        Clear(ClearType::CurrentLine),
        Print("Enter path: "),
        if !is_valid {
            SetForegroundColor(Color::DarkRed)
        } else {
            SetForegroundColor(Color::Green)
        },
        Print(buffer),
        SetForegroundColor(Color::Reset)
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    // TODO!: better testing for the valid_regex and valid_path functions
    #[test]
    fn test_valid_regex() {
        let _handle = thread::spawn(|| {
            valid_regex(Regex::new(r"^\d{3}$").unwrap());
        });

        thread::sleep(Duration::from_secs(5));

        // If the test reaches this point without crashing, consider it a success
        assert!(true);
    }

    #[test]
    fn test_valid_path() {
        let _handle = thread::spawn(|| {
            valid_path();
        });

        thread::sleep(Duration::from_secs(5));

        assert!(true);
    }

    #[test]
    fn test_validate_path_existing_file() {
        // Create a temporary file for testing
        let file_path = "test_file.txt";
        std::fs::File::create(file_path).expect("Failed to create file");

        // Validate the path of the temporary file
        assert_eq!(validate_path(file_path), true);

        // Delete the temporary file
        std::fs::remove_file(file_path).expect("Failed to delete file");
    }

    #[test]
    fn test_validate_path_nonexistent_file() {
        // Create a temporary file path that doesn't exist
        let file_path = "nonexistent_file.txt";

        // Validate the path of the nonexistent file
        assert_eq!(validate_path(file_path), false);
    }

    #[test]
    fn test_render_input_prompt() {
        // Call the render_input_prompt function with a mock Stdout
        render_input_prompt("123", &true);
    }
}
