use crossterm::style::{ContentStyle, Print, ResetColor, SetStyle};
use crossterm::{cursor, execute, terminal};
use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::frames::spinner::Frames;

/// Spinner struct encapsulating the spinner animation.
pub struct Spinner {
    frames: Frames,
    should_stop: Arc<Mutex<bool>>,
    text: Arc<Mutex<Option<String>>>,
    animation_style: Arc<Mutex<ContentStyle>>,
    text_style: Arc<Mutex<ContentStyle>>,
    end_sequence: Arc<Mutex<Option<String>>>,
    cleanup_on_exit: Arc<Mutex<bool>>,
}

impl Spinner {
    /// Creates a new instance of Spinner.
    pub fn new(
        frames: Frames,
        should_stop: Arc<Mutex<bool>>,
        text: Arc<Mutex<Option<String>>>,
        animation_style: Arc<Mutex<ContentStyle>>,
        text_style: Arc<Mutex<ContentStyle>>,
        end_sequence: Arc<Mutex<Option<String>>>,
        cleanup_on_exit: Arc<Mutex<bool>>,
    ) -> Self {
        Spinner {
            frames,
            should_stop,
            text,
            animation_style,
            text_style,
            end_sequence,
            cleanup_on_exit,
        }
    }

    /// Runs the spinner animation.
    pub fn run(&self) {
        let mut frame_index = 0;
        let longest_frame_len = self
            .frames
            .frames
            .iter()
            .map(|frame| frame.len())
            .max()
            .unwrap_or(0);

        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            cursor::Hide
        )
            .unwrap(); // hide cursor

        while !*self.should_stop.lock().unwrap() {
            self.display_frame(&mut frame_index);
            thread::sleep(Duration::from_millis(self.frames.speed_ms));
        }

        self.display_end_sequence();
        self.cleanup(longest_frame_len);
    }

    /// Displays a frame of the spinner animation.
    fn display_frame(&self, frame_index: &mut usize) {
        let frame = &self.frames.frames[*frame_index];

        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            SetStyle(*self.animation_style.lock().unwrap()), // set animation color
            Print(frame),
            Print("  "),
            SetStyle(*self.text_style.lock().unwrap()), // set text color
            Print(self.text.lock().unwrap().as_ref().unwrap_or(&"".to_string())),
            ResetColor, // reset colors
        )
            .unwrap();

        *frame_index = (*frame_index + 1) % self.frames.frames.len();
    }

    /// Displays the end sequence of the spinner animation.
    fn display_end_sequence(&self) {
        if let Some(end_seq) = &*self.end_sequence.lock().unwrap() {
            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::CurrentLine),
                cursor::MoveToColumn(0),
                ResetColor,
                Print(end_seq),
                Print("  "),
                SetStyle(*self.text_style.lock().unwrap()),
                Print(self.text.lock().unwrap().as_ref().unwrap_or(&"".to_string())),
                ResetColor,
                Print("\n"),
            )
                .unwrap();
        }
    }

    /// Cleans up after the spinner animation.
    fn cleanup(&self, longest_frame_len: usize) {
        if *self.cleanup_on_exit.lock().unwrap() {
            let clear_length = " ".repeat(longest_frame_len);
            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::CurrentLine),
                cursor::MoveToColumn(0),
                Print(&clear_length),
                Print("\r"),
            )
                .unwrap();
        }

        execute!(
            stdout(),
            cursor::Show, // show cursor
        )
            .unwrap();
    }
}
