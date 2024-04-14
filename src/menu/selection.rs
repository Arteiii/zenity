//! selection menus

use std::sync::{Arc, Mutex};

/// Represents an option in a selection menu
#[derive(Clone)]
pub struct Option {
    /// Text to display for this option
    pub text: String,
    /// Additional notes or information about this option
    pub notes: String,
}

/// Represents a selection menu
#[derive(Clone)]
pub struct SelectionMenu {
    /// List of options along with the correct return id
    pub options: Arc<Mutex<Vec<Option>>>,
}

impl Default for SelectionMenu {
    /// Creates a new default selection menu
    fn default() -> Self {
        todo!()
    }
}

impl SelectionMenu {
    /// Creates a new selection menu with the given option
    pub fn new(option: Option) -> Self {
        SelectionMenu {
            options: Arc::new(Mutex::new(vec![option.clone()])),
        }
    }

    /// Adds a new option to the selection menu
    pub fn add_option(&mut self, new_option: Option) {
        self.options.lock().unwrap().push(new_option);
    }
}