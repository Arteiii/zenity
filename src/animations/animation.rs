use crossterm::{cursor, execute, style::{Color, Print, ResetColor, SetForegroundColor}, terminal};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::frames::spinner::Frames;

/// Prints the given text without buffering.
fn print_flush(text: &str) {
    print!("{}", text);
    stdout().flush().unwrap();
}

/// moves the cursor back and prints the given text without buffering
fn go_back_print_flush(text: &str) {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0),
        Print(text),
    )
        .unwrap();
}

/// A loading animation that runs in a separate thread.
pub fn spinner_animation(frames: &Frames, should_stop: Arc<Mutex<bool>>) {
    let mut frame_index = 0;
    let longest_frame_len = frames.frames.iter().map(|frame| frame.len()).max().unwrap_or(0);

    execute!(stdout(), cursor::Hide).unwrap(); // Hide cursor

    while !*should_stop.lock().unwrap() {
        let frame = &frames.frames[frame_index];
        go_back_print_flush(frame);

        thread::sleep(Duration::from_millis(frames.speed_ms));
        print_flush("\r");

        frame_index = (frame_index + 1) % frames.frames.len();
    }

    let clear_length = " ".repeat(longest_frame_len);
    go_back_print_flush(&clear_length);

    execute!(stdout(), cursor::Show).unwrap(); // Show cursor
    print_flush("\r"); // Set to beginning of line
}


// TODO: implement loading bar animation
