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

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use regex::Regex;

use crate::color::ENABLE_COLOR;
use crate::menu::handle_key_input;
use crate::style::{Color, Print, SetForegroundColor};
use crate::terminal::console_render::raw_mode_wrapper;

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
    /// NOT WORKING yet will add asap
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
            false_note: Some("Please enter a valid path!".to_string()),
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
    pub fn set_regex(mut self, regex: Regex) -> Self {
        self.regex = Some(regex);

        self
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
    pub fn set_note(mut self, valid: &str, invalid: &str) -> Self {
        self.true_note = Some(valid.to_string());
        self.false_note = Some(invalid.to_string());

        self
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

/// Represents an input field with validation requirements and optional default value.
pub struct Input {
    /// The title or prompt displayed for the input field.
    title: String,
    /// The validation requirements for the input field.
    requirements: Vec<Requirements>,
    /// The default value that can be accepted by pressing Enter.
    default: Option<String>,
    /// Indicates whether the input can be forced without meeting validation requirements.
    allow_force: bool,
}

impl Input {
    /// Creates a new input field with the specified title and validation requirements.
    ///
    /// # Arguments
    ///
    /// * `title` - The title or prompt for the input field.
    /// * `req` - The validation requirement for the input field more can be added with the ``add_requirement`` method
    ///
    /// # Returns
    ///
    /// A new `Input` instance with the given title and validation requirements.
    pub fn new(title: &str, req: Requirements) -> Self {
        let reqs = vec![req];

        Input {
            title: title.to_string(),
            requirements: reqs,
            default: None,
            allow_force: false,
        }
    }

    /// Starts the input process, displaying the prompt and handling user input.
    ///
    /// This method prompts the user for input, validates it according to the specified requirements,
    /// and returns the validated input as a boxed string.
    ///
    /// # Returns
    ///
    /// A boxed string containing the validated input.
    pub fn start(&self) -> Box<String> {
        let mut force: bool = false;
        let mut buffer = String::new();

        // Initialize vectors to store validation status and notes
        let mut validation_status = Vec::new();
        let mut notes = Vec::new();

        loop {
            raw_mode_wrapper!(self.render_input_prompt(
                &buffer,
                validation_status.iter().all(|&status| status),
                &notes,
            ));

            let result = handle_key_input(&mut buffer, &mut force);
            // Perform validation for each requirement and store results
            validation_status.clear();
            notes.clear();

            for req in &self.requirements {
                let mut path_valid = true;

                if req.path {
                    path_valid = Self::validate_path(&buffer);
                }

                let regex_valid = req
                    .regex
                    .as_ref()
                    .map_or(true, |regex| Self::validate_regex(&buffer, regex));

                // Push the validation status of each requirement
                validation_status.push(path_valid && regex_valid);

                // Store notes for each requirement
                notes.push(if validation_status.last() == Some(&true) {
                    req.true_note.clone()
                } else {
                    req.false_note.clone()
                });
            }

            if result {
                // Check if all requirements are satisfied
                if validation_status.iter().all(|&status| status) {
                    break;
                } else if self.default.is_some() && buffer.is_empty() {
                    buffer = self.default.clone().unwrap();
                    break;
                }
            }

            if force && self.allow_force {
                break;
            }
        }

        // clear the line before exit
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 4),
            Clear(ClearType::FromCursorDown),
            cursor::Show,
        )
        .unwrap();

        Box::new(buffer)
    }

    /// Adds a new requirement to the input.
    pub fn add_requirement(mut self, requirement: Requirements) -> Self {
        self.requirements.push(requirement);
        self
    }

    /// Enables the ability to bypass validation requirements and force input submission.
    /// This can be triggered by pressing SHIFT + Enter.
    ///
    /// **Note:**
    /// - This feature may not work in all terminal environments. Refer to issue [#685](https://github.com/crossterm-rs/crossterm/issues/685) for more information.
    pub fn allow_force(mut self) -> Self {
        self.allow_force = true;
        self
    }

    /// Sets the default value, which can be accepted by pressing Enter.
    ///
    /// Pressing Enter without typing anything will accept the default value.
    pub fn set_default(mut self, default: &str) -> Self {
        self.default = Some(default.to_string());
        self
    }

    // helper functions:
    #[inline]
    fn validate_path(path: &str) -> bool {
        // useless function but might change something here later...
        Path::new(path).exists()
    }

    #[inline]
    fn validate_regex(buffer: &str, regex: &Regex) -> bool {
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

    fn render_input_prompt(&self, buffer: &str, valid: bool, notes: &[Option<String>]) {
        // clear the line before rendering
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 4),
            Clear(ClearType::CurrentLine),
            cursor::Hide,
        )
        .unwrap();

        // determine color based on validity and color enablement
        let (text_color, content) = if !buffer.is_empty() || self.default.is_none() {
            let text_color = if *ENABLE_COLOR {
                if !valid {
                    Color::DarkRed
                } else {
                    Color::Green
                }
            } else {
                Color::Reset
            };
            (text_color, buffer.to_string())
        } else {
            let text_color = if *ENABLE_COLOR {
                Color::Grey
            } else {
                Color::Reset
            };
            (text_color, self.default.clone().unwrap_or_default())
        };

        // render the prompt
        execute!(
            io::stdout(),
            Print(&self.title),
            cursor::MoveToNextLine(1),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(text_color),
            Print(content),
        )
        .unwrap();

        // if using default, indicate it
        if self.default.is_some() && buffer.is_empty() {
            execute!(io::stdout(), Print(" (Default)")).unwrap();
        }

        // reset color
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Reset),
            cursor::SavePosition,
            cursor::MoveToNextLine(2)
        )
        .unwrap();

        if *ENABLE_COLOR {
            execute!(io::stdout(), SetForegroundColor(Color::DarkGrey)).unwrap();
        }

        // Print notes
        for note in notes.iter() {
            match note {
                Some(note_str) => {
                    execute!(
                        io::stdout(),
                        cursor::MoveToNextLine(1),
                        Print("- "),
                        Print(note_str)
                    )
                    .unwrap();
                }
                None => {
                    execute!(
                        io::stdout(),
                        cursor::MoveToNextLine(1),
                        Clear(ClearType::CurrentLine),
                        cursor::MoveToPreviousLine(1),
                    )
                    .unwrap();
                }
            }
        }

        if self.allow_force && !buffer.is_empty() && !valid {
            execute!(
                io::stdout(),
                cursor::MoveToNextLine(2),
                Print("Press SHIFT + Enter to force input"),
            )
            .unwrap();
        } else {
            execute!(
                io::stdout(),
                cursor::MoveToNextLine(2),
                Clear(ClearType::CurrentLine),
                cursor::MoveToNextLine(2),
            )
            .unwrap();
        }

        if self.default.is_some() && buffer.is_empty() {
            execute!(
                io::stdout(),
                cursor::MoveToNextLine(2),
                Print("Press Enter to accept default"),
            )
            .unwrap();
        } else {
            execute!(
                io::stdout(),
                cursor::MoveToNextLine(2),
                Clear(ClearType::CurrentLine),
                cursor::MoveToNextLine(2),
            )
            .unwrap();
        }

        // reset color
        execute!(
            io::stdout(),
            cursor::RestorePosition,
            SetForegroundColor(Color::Reset),
            cursor::Show,
        )
        .unwrap();
    }
}
