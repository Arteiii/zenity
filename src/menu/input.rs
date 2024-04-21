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
    terminal::{Clear, ClearType},
};
use regex::Regex;

use crate::color::ENABLE_COLOR;
use crate::style::{Color, Print, SetForegroundColor};

pub struct Requirements {
    /// regex to match (optional if ``path`` is true)
    regex: Option<Regex>,

    /// if the input needs to be a valid path, if its valid, the ``regex`` will be applied to the name and extension
    path: bool,

    /// allow creating the path if its doesn't exist yet the ``regex`` still needs to match
    ///
    /// this only works if ``path`` is true
    allow_creating: bool,

    /// note to display if condition matches
    true_note: Option<String>,

    /// note to display while the condition doesn't match
    false_note: Option<String>,
}

impl Default for Requirements {
    fn default() -> Self {
        Requirements::path()
    }
}


impl Requirements {
    pub fn regex(regex: Regex) -> Self {
        Requirements {
            regex: Some(regex),
            path: false,
            allow_creating: false,
            true_note: None,
            false_note: None,
        }
    }


    pub fn path()-> Self {
        Requirements {
            regex: None,
            path: true,
            allow_creating: false,
            true_note: None,
            false_note: None,
        }
    }

    pub fn set_regex(mut self, regex: Regex)  {
        self.regex = Some(regex);
    }


    pub fn set_note(mut self, valid: &str, invalid: &str){
        self.true_note = Some(valid.to_string());
        self.false_note = Some(invalid.to_string());
    }

    pub fn allow_creation(mut self){
        self.allow_creating = true;
    }
}


pub struct Input {
    title: String,
    requirements: Vec<Requirements>,
    require_existing_path: bool,
    allow_path_creation: bool,
}

impl Default for Input {
    fn default() -> Self {
        // TODO!
    }
}

impl Input {
    // TODO! add methods

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
            if *ENABLE_COLOR {
                if !is_valid {
                    SetForegroundColor(Color::DarkRed)
                } else {
                    SetForegroundColor(Color::Green)
                }
            } else {
                SetForegroundColor(Color::Reset)
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
            if *ENABLE_COLOR {
                SetForegroundColor(Color::Grey)
            } else {
                SetForegroundColor(Color::Reset)
            },
            Print(default.unwrap()),
            Print(" (Default)"),
        )
        .expect("execute print default failed");
    }
    execute!(io::stdout(), SetForegroundColor(Color::Reset),).unwrap();
}
