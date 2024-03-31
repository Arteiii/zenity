use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};
use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::frames::spinner::Frames;

/// a loading animation that runs in a separate thread.
pub fn spinner_animation(
    frames: &Frames,
    should_stop: Arc<Mutex<bool>>,
    text: Arc<Mutex<Option<String>>>,
    animation_color: Arc<Mutex<Color>>,
    text_color: Arc<Mutex<Color>>,
) {
    let mut frame_index = 0;
    let longest_frame_len = frames
        .frames
        .iter()
        .map(|frame| frame.len())
        .max()
        .unwrap_or(0);

    execute!(stdout(), cursor::Hide).unwrap(); // hide cursor

    while !*should_stop.lock().unwrap() {
        let frame = &frames.frames[frame_index];

        // clear the current line, move the cursor to the beginning of the line, print the frame, and flush stdout
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            SetForegroundColor(*animation_color.lock().unwrap()), // set animation color
            Print(frame),
            Print("  "),
            SetForegroundColor(*text_color.lock().unwrap()), // set text color
            Print(text.lock().unwrap().as_ref().unwrap_or(&"".to_string())),
            ResetColor, // reset colors
            Print("\r"),
        )
        .unwrap();

        thread::sleep(Duration::from_millis(frames.speed_ms));

        frame_index = (frame_index + 1) % frames.frames.len();
    }

    let clear_length = " ".repeat(longest_frame_len);

    // clear the line after the animation finishes
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0),
        Print(&clear_length),
        Print("\r"),
        cursor::Show, // show cursor
    )
    .unwrap();
}
