//! Input Validation Widgets
//!
//! **Note:** This module is a work in progress,
//! and breaking changes could be made soon without increasing the major version
//! for different reasons, such as improvements or bug fixes.
//!
//!
//! This module provides functions for validating user input with various criteria,
//! such as regex patterns or file paths
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

use std::io;
use std::path::{Path, PathBuf};

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use regex::Regex;

use crate::menu::handle_key_input;
use crate::style::{Color, Print, SetForegroundColor};

macro_rules! input_loop {
    ($title:expr, $buffer:expr, $validate:expr, $default:expr, $allow_force:expr) => {
        let mut force: bool = false;

        loop {
            render_input_prompt($title, &$buffer, &$validate, $default);

            if handle_key_input(&mut $buffer, &mut force) {
                if !$buffer.is_empty() && $validate {
                    break;
                } else if $default.is_some() && $buffer.is_empty() {
                    $buffer = $default.unwrap().to_string();
                    break;
                }
            }

            if force && $allow_force {
                break;
            }
        }
    };
}

macro_rules! raw_mode_wrapper {
    ($content:expr) => {
        enable_raw_mode().expect("Failed to enable raw-mode");

        $content;

        disable_raw_mode().expect("Failed to disable raw-mode");
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            Clear(ClearType::FromCursorDown),
            cursor::DisableBlinking
        )
        .unwrap();
    };
}

/// Validates and returns a string that matches the specified regex pattern.
///
/// This function prompts the user to enter input and validates the input against the provided
/// regex pattern. It continues to prompt the user until the entered input matches the regex pattern.
/// The function returns the validated input as a string.
///
/// If `default` is provided and the user enters an empty string, the default value will be used.
///
/// Note: The `allow_force` option is currently not fully supported due to console issues. See
/// [this issue](https://github.com/crossterm-rs/crossterm/issues/685) for more details. However,
/// users can force input by pressing Shift+Enter. Pressing Shift+Enter will clear the full input.
///
///
/// # Arguments
///
/// * `regex` - A `Regex` object representing the regex pattern to match against the user input.
/// * `default` - An optional default value to be used if the user enters an empty string.
/// * `allow_force` - A boolean indicating whether to allow the user to force input (not fully supported).
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
/// let input = valid_regex(regex, Some("default_value"), false);
///
/// println!("Valid input: {}", input);
/// ```
pub fn valid_regex(title: &str, regex: Regex, default: Option<&str>, allow_force: bool) -> String {
    let mut buffer = String::new();

    raw_mode_wrapper!(input_loop!(
        title,
        buffer,
        validate_input(&buffer, &regex),
        default,
        allow_force
    ));

    buffer
}

/// Validates and returns a `PathBuf` representing the entered path.
///
/// This function prompts the user to enter a path and validates the input. If the entered path is valid,
/// it returns a `PathBuf` containing the path. Otherwise, it continues prompting the user until a valid
/// path is entered.
///
/// If `default` is provided and the user enters an empty string, the default value will be used.
///
/// Note: The `allow_force` option is currently not fully supported due to console issues. See
/// [this issue](https://github.com/crossterm-rs/crossterm/issues/685) for more details. However,
/// users can force input by pressing Shift+Enter. Pressing Shift+Enter will clear the full input.
///
/// # Arguments
///
/// * `default` - An optional default value to be used if the user enters an empty string.
/// * `allow_force` - A boolean indicating whether to allow the user to force input (not fully supported).
///
/// # Returns
///
/// A `Box<PathBuf>` representing the validated path entered by the user.
///
/// # Examples
///
/// ```rust,ignore
/// use zenity::menu::input::valid_path;
///
/// let path = valid_path(Some("/home/user"), true);
/// println!("Entered path: {:?}", path);
/// ```
pub fn valid_path(title: &str, default: Option<&str>, allow_force: bool) -> Box<PathBuf> {
    let mut buffer = String::new();

    raw_mode_wrapper!(input_loop!(
        title,
        buffer,
        validate_path(&buffer),
        default,
        allow_force
    ));

    let path = PathBuf::from(buffer);

    Box::new(path)
}

#[inline]
fn validate_path(path: &str) -> bool {
    // useless function but might change something here later...
    Path::new(path).exists()
}

#[inline]
fn validate_input(buffer: &str, regex: &Regex) -> bool {
    if regex.is_match(buffer) {
        true
    } else {
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 5),
            Clear(ClearType::CurrentLine)
        )
        .unwrap();
        false
    }
}

fn render_input_prompt(title: &str, buffer: &str, is_valid: &bool, default: Option<&str>) {
    execute!(
        io::stdout(),
        cursor::MoveTo(0, 4),
        Clear(ClearType::CurrentLine),
    )
    .unwrap();
    if !buffer.is_empty() || default.is_none() {
        execute!(
            io::stdout(),
            Print(title),
            cursor::MoveToNextLine(1),
            Clear(ClearType::CurrentLine),
            if !is_valid {
                SetForegroundColor(Color::DarkRed)
            } else {
                SetForegroundColor(Color::Green)
            },
            Print(buffer),
        )
        .expect("execute print buffer failed");
    } else {
        execute!(
            io::stdout(),
            Print(title),
            cursor::MoveToNextLine(1),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Grey),
            Print(default.unwrap()),
            Print(" (Default)"),
        )
        .expect("execute print default failed");
    }
    execute!(io::stdout(), SetForegroundColor(Color::Reset),).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path_existing_file() {
        // Create a temporary file for testing
        let file_path = "test_file.txt";
        std::fs::File::create(file_path).expect("Failed to create file");

        // Validate the path of the temporary file
        assert!(validate_path(file_path));

        // Delete the temporary file
        std::fs::remove_file(file_path).expect("Failed to delete file");
    }

    #[test]
    fn test_validate_path_nonexistent_file() {
        // Create a temporary file path that doesn't exist
        let file_path = "nonexistent_file.txt";

        // Validate the path of the nonexistent file
        assert!(!validate_path(file_path));
    }

    #[test]
    fn test_render_input_prompt() {
        // Call the render_input_prompt function with a mock Stdout
        render_input_prompt("Title", "123", &true, Some("Default stuff"));
    }

    #[test]
    fn test_validate_input() {
        // Call the render_input_prompt function with a mock Stdout
        assert!(validate_input("123", &Regex::new(r"^\d{3}$").unwrap()));
        assert!(!validate_input("abc", &Regex::new(r"^\d{3}$").unwrap()));
    }
}
