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

use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use regex::Regex;


use crate::{style::{Color, Print, SetForegroundColor}, terminal::console_render::raw_mode_wrapper};


/// Represents requirements for validating user input
///
/// # Examples
///
/// ```
/// use zenity::menu::input::Requirements;
///
/// // Create default requirements for validating paths
/// let default_requirements = Requirements::default();
/// ```
pub struct Requirements {
    /// Regex to match (optional if `path` is true)
    regex: Option<Regex>,

    /// If the input needs to be a valid path
    /// 
    /// If valid, the `regex` will be applied to the name and extension
    path: bool,

    /// Allow creating the path if it doesn't exist yet
    /// 
    /// **NOTES**  
    /// - The `regex` still needs to match
    /// - This only works if `path` is true
    allow_creating: bool,

    /// Note to display if the condition matches
    true_note: Option<String>,

    /// Note to display while the condition doesn't match
    false_note: Option<String>,
}

impl Default for Requirements {
    /// Creates default requirements for validating paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::menu::input::Requirements;
    ///
    /// // Create default requirements for validating paths
    /// let default_requirements = Requirements::default();
    /// ```
    fn default() -> Self {
        Requirements::path()
    }
}
impl Requirements {
    /// Creates requirements with a specific regex
    ///
    /// # Examples
    ///
    /// ```
    /// use regex::Regex;
    /// use zenity::menu::input::Requirements;
    ///
    /// // Create requirements with a specific regex
    /// let regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    /// let regex_requirements = Requirements::regex(regex);
    /// ```
    pub fn regex(regex: Regex) -> Self {
        Requirements {
            regex: Some(regex),
            path: false,
            allow_creating: false,
            true_note: None,
            false_note: None,
        }
    }

    /// Creates requirements for validating paths
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::menu::input::Requirements;
    ///
    /// // Create requirements for validating paths
    /// let path_requirements = Requirements::path();
    /// ```
    pub fn path() -> Self {
        Requirements {
            regex: None,
            path: true,
            allow_creating: false,
            true_note: None,
            false_note: None,
        }
    }

    /// Sets the regex for the requirements
    ///
    /// # Examples
    ///
    /// ```
    /// use regex::Regex;
    /// use zenity::menu::input::Requirements;
    ///
    /// // Create requirements for validating paths
    /// let mut path_requirements = Requirements::path();
    ///
    /// // Set a custom regex for path validation
    /// let regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    /// path_requirements.set_regex(regex);
    /// ```
    pub fn set_regex(mut self, regex: Regex) {
        self.regex = Some(regex);
    }


    /// Sets notes to display based on validity
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::menu::input::Requirements;
    ///
    /// // Create requirements for validating paths
    /// let mut path_requirements = Requirements::path();
    ///
    /// // Set notes to display based on validity
    /// path_requirements.set_note("Valid path.", "Invalid path.");
    /// ```
    pub fn set_note(mut self, valid: &str, invalid: &str) {
        self.true_note = Some(valid.to_string());
        self.false_note = Some(invalid.to_string());
    }


    /// Allows creating the path if it doesn't exist yet
    ///
    /// # Examples
    ///
    /// ```
    /// use zenity::menu::input::Requirements;
    ///
    /// // Create requirements for validating paths
    /// let mut path_requirements = Requirements::path();
    ///
    /// // Allow creating the path if it doesn't exist yet
    /// path_requirements.allow_creation();
    /// ```
    pub fn allow_creation(mut self) {
        self.allow_creating = true;
    }
}


pub struct Input {
    title: Arc<Mutex<String>>,
    requirements: Arc<Mutex<Vec<Requirements>>>,
}


impl Input {
    pub fn new(title: &str, req: Requirements) -> Self {
        let reqs = vec![req];
        
        Input {
            title: Arc::new(Mutex::new(title.to_string())),
            requirements: Arc::new(Mutex::new(reqs)),
        }
    }
    
    fn 
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

fn render_input_prompt(
    title: &str,
    buffer: &str,
    is_valid: &bool,
    default: Option<&str>,
    enable_color: bool,
) {
    // clear the line before rendering
    execute!(
        io::stdout(),
        cursor::MoveTo(0, 4),
        Clear(ClearType::CurrentLine)
    )
    .unwrap();

    // determine color based on validity and color enablement
    let (text_color, content) = if !buffer.is_empty() || default.is_none() {
        let text_color = if enable_color {
            if !is_valid {
                Color::DarkRed
            } else {
                Color::Green
            }
        } else {
            Color::Reset
        };
        (text_color, buffer)
    } else {
        let text_color = if enable_color {
            Color::Grey
        } else {
            Color::Reset
        };
        (text_color, default.unwrap_or_default())
    };

    // render the prompt
    execute!(
        io::stdout(),
        Print(title),
        cursor::MoveToNextLine(1),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(text_color),
        Print(content),
    )
    .unwrap();

    // if using default, indicate it
    if default.is_some() {
        execute!(io::stdout(), Print(" (Default)")).unwrap();
    }

    // reset color
    execute!(io::stdout(), SetForegroundColor(Color::Reset)).unwrap();
}
