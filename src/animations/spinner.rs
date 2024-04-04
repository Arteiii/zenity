use std::collections::HashMap;
use std::io::{stdout, Stdout};
use std::sync::{Arc, Mutex};

use crossterm::{cursor, queue, terminal};
use crossterm::style::{ContentStyle, Print, ResetColor, SetStyle};

use crate::spinner::Frames;
use crossterm::{cursor, queue, terminal};

struct LocatedFrame {
    frame: &'static str,
    column: u16,
    row: u16,
}

/// struct holding multiple spinners
pub struct MultiSpinner {}

impl MultiSpinner {
    fn render_frame(mut stdout: Stdout, row: u16, column: u16, frame: &str) {
        queue!(
            stdout,
            cursor::RestorePosition,
            cursor::MoveTo(column, row),
            terminal::Clear(terminal::ClearType::CurrentLine),
            Print(frame),
        )
        .unwrap();
    }
}
