use std::collections::HashMap;
use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, Mutex};

use crossterm::{cursor, queue, terminal};
use crossterm::{cursor, queue, terminal};
use crossterm::style::{ContentStyle, Print, ResetColor, SetStyle};
use utils_arteii_rs::vector_operations::iterators;

use crate::spinner::Frames;

/// struct holding multiple spinners
pub struct MultiSpinner {}

impl MultiSpinner {
    fn render_frame(mut stdout: &Stdout, frame: &str) {
        queue!(stdout, Print(frame),).unwrap();
        stdout.flush().unwrap();
    }

    fn prepare_frame(){
        iterators::balanced_iterator();
        let frame = "";

        render_frame(frame);
    }

}
