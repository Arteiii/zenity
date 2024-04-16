//! selection menus work in progress

use crate::style::StyledString;
use crate::terminal::{console_cursor, console_render};

/// Represents an option in a selection menu
#[derive(Clone, Debug, PartialEq)] 
pub struct MenuOption {
    /// Text to display for this option
    pub text: StyledString,
    /// Additional notes or information about this option
    pub notes: StyledString,
}

/// Represents a selection menu
#[derive(Clone)]
pub struct SelectionMenu {
    /// the title for the
    pub title: StyledString,

    /// List of options along with the correct return id
    pub options: Vec<MenuOption>,
}

impl Default for SelectionMenu {
    /// Creates a new default selection menu
    fn default() -> Self {
        todo!()
    }
}

impl SelectionMenu {
    /// render the single selection
    pub fn single(self) -> MenuOption{
        let title = self.title.clone();

        console_cursor::save_hide_cursor();
        
        console_render::render_styled_line(1, &[title]);
        
        console_cursor::next_line(4);

        for (index, option) in self.options.iter().enumerate().clone() {
            console_render::render_styled_line((index + 2).try_into().unwrap(), &[option.text.clone()]);
        }
        
        self.options[0].clone()
    }
}
